mod scraper;

use std::{collections::HashMap, sync::Arc};

use lazy_static::lazy_static;
use regex::Regex;
use rxrust::prelude::*;

use crate::{
    sinks::{file::FileSink, Sink},
    sources::browser::headless_chrome::{UrlHtml, UrlHtmlObservable},
};

use self::scraper::{KickertoolData, Table};

type KickertoolDataObservable =
    impl Clone + Observable<Item = KickertoolData, Err = ()> + SharedObservable;

pub struct Kickertool {
    team_subscriptions: HashMap<(u8, usize), Box<dyn SubscriptionLike>>,
    standings_subscription: Option<Box<dyn SubscriptionLike>>,
    kickertool_data_observable: KickertoolDataObservable,
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
        .flat_map(|html| observable::of_option(KickertoolData::from_qualification_display(html)))
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
        };
        s.standings_subscribe();
        s.team_subscribe(1, 1);
        s.team_subscribe(1, 2);

        s
    }

    fn standings_subscribe(&mut self) {
        Self::unsubscribe(self.standings_subscription.as_mut());

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

        self.standings_subscription = (Box::new(s) as Box<dyn SubscriptionLike>).into();
    }

    fn get_table_observable(&self, number: u8) -> TableObservable {
        self.kickertool_data_observable
            .clone()
            .filter_map(move |data: KickertoolData| {
                data.tables.into_iter().find(|table| table.number == number)
            })
    }

    fn team_subscribe(&mut self, table_number: u8, team_number: usize) {
        self.team_unsubscribe(table_number, team_number);

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

        let mut subscription = self.get_team_subscription_mut(table_number, team_number);
        subscription.replace(&mut (Box::new(s) as Box<dyn SubscriptionLike>));
    }

    fn team_unsubscribe(&mut self, table_number: u8, team_number: usize) {
        Self::unsubscribe(self.get_team_subscription_mut(table_number, team_number));
    }

    fn unsubscribe(mut subscription: Option<&mut Box<dyn SubscriptionLike>>) {
        if let Some(subscription) = subscription.take() {
            subscription.unsubscribe();
        }
    }

    fn get_team_subscription_mut(
        &mut self,
        table_number: u8,
        team_number: usize,
    ) -> Option<&mut Box<dyn SubscriptionLike>> {
        self.team_subscriptions
            .get_mut(&(table_number, team_number))
    }
}

impl Drop for Kickertool {
    fn drop(&mut self) {
        self.team_unsubscribe(1, 1);
        self.team_unsubscribe(1, 2);
        Self::unsubscribe(self.standings_subscription.as_mut());
    }
}
