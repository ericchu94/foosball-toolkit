mod scraper;

use std::{collections::HashMap, sync::Arc};

use kickertool_data::*;
use lazy_static::lazy_static;
use regex::Regex;
use rxrust::prelude::*;

use crate::{
    sinks::{file::FileSink, http_post::HttpPostSink, Sink},
    sources::browser::headless_chrome::{UrlHtml, UrlHtmlObservable},
};

use self::scraper::kickertool_data_from_qualification_display;

type KickertoolDataObservable =
    impl Clone + Observable<Item = KickertoolData, Err = ()> + SharedObservable;

type DataSubscription = impl SubscriptionLike;
type StandingsSubscription = impl SubscriptionLike;
type TeamSubscription = impl SubscriptionLike;

pub struct Kickertool {
    team_subscriptions: HashMap<(u8, usize), SubscriptionGuard<TeamSubscription>>,
    standings_subscription: Option<SubscriptionGuard<StandingsSubscription>>,
    kickertool_data_observable: KickertoolDataObservable,
    data_subscription: Option<SubscriptionGuard<DataSubscription>>,
}

fn get_kickertool_data_observable(
    url_html_observable: UrlHtmlObservable,
) -> KickertoolDataObservable {
    url_html_observable
        .filter_map(|url_html: Arc<UrlHtml>| {
            lazy_static! {
                static ref QUALIFICATION_DISPLAY_URL_REGEX: Regex =
                    Regex::new(r"https://app\.kickertool\.de/display/#/.*/tournament").unwrap();
            }
            if QUALIFICATION_DISPLAY_URL_REGEX.is_match(&url_html.url) {
                Some(url_html.html.clone())
            } else {
                None
            }
        })
        .distinct_until_changed()
        .flat_map(|html| observable::of_option(kickertool_data_from_qualification_display(html)))
        .tap(|data| println!("Parsed data: {:?}", data))
        .distinct_until_changed()
        .tap(|data| println!("Distinct data: {:?}", data))
        .share()
        .into_shared()
}

type TableObservable = impl Observable<Item = Table, Err = ()> + Clone + SharedObservable;

impl Kickertool {
    pub fn new(url_html_observable: UrlHtmlObservable) -> Self {
        let kickertool_data_observable = get_kickertool_data_observable(url_html_observable);
        let mut s = Self {
            team_subscriptions: HashMap::new(),
            standings_subscription: None,
            kickertool_data_observable,
            data_subscription: None,
        };
        s.standings_subscribe();
        s.team_subscribe(1, 1);
        s.team_subscribe(1, 2);
        s.data_subscribe();

        s
    }
    fn data_subscribe(&mut self) {
        let sink = HttpPostSink::new("http://localhost:8000/data");

        let s = self
            .kickertool_data_observable
            .clone()
            .into_shared()
            .subscribe(move |data| sink.sink(data));

        let guard = s.unsubscribe_when_dropped();

        self.data_subscription = Some(guard);
    }

    fn standings_subscribe(&mut self) {
        let sink = FileSink::new("standings.txt");

        let s = self
            .kickertool_data_observable
            .clone()
            .map(|data| data.standings.join("\n"))
            .distinct_until_changed()
            .tap(move |standings| {
                println!("{standings}");
            })
            .into_shared()
            .subscribe(move |standings| sink.sink(standings));

        self.standings_subscription = Some(s.unsubscribe_when_dropped());
    }

    fn get_table_observable(&self, number: u8) -> TableObservable {
        self.kickertool_data_observable
            .clone()
            .filter_map(move |data: KickertoolData| {
                data.tables.into_iter().find(|table| table.number == number)
            })
    }

    fn team_subscribe(&mut self, table_number: u8, team_number: usize) {
        let sink = FileSink::new(format!("table{table_number}-team{team_number}.txt"));

        let s = self
            .get_table_observable(1)
            .map(move |data| match team_number {
                1 => data.r#match.team1,
                2 => data.r#match.team2,
                _ => unreachable!(),
            })
            .distinct_until_changed()
            .tap(move |team| println!("Table{table_number} Team{team_number}: {team}"))
            .into_shared()
            .subscribe(move |standings| sink.sink(standings));

        self.team_subscriptions
            .insert((table_number, team_number), s.unsubscribe_when_dropped());
    }
}
