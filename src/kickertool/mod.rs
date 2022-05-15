mod scraper;

use std::sync::Arc;

use rxrust::prelude::*;

use crate::{
    sinks::{file::FileSink, Sink},
    sources::browser::headless_chrome::{UrlHtml, UrlHtmlObservable},
};

use self::scraper::{KickertoolData, Table};

type KickertoolDataObservable =
    impl Clone + Observable<Item = KickertoolData, Err = ()> + SharedObservable;

pub struct Kickertool {
    team_subscriptions: [Option<Box<dyn SubscriptionLike>>; 2],
    standings_subscription: Option<Box<dyn SubscriptionLike>>,
    kickertool_data_observable: KickertoolDataObservable,
}

fn get_kickertool_data_observable(
    url_html_observable: UrlHtmlObservable,
) -> KickertoolDataObservable {
    url_html_observable
        .filter_map(|url_html: Arc<UrlHtml>| {
            if url_html
                .url
                .starts_with("https://app.kickertool.de/tournament/")
            {
                Some(url_html.html.clone())
            } else {
                None
            }
        })
        .distinct_until_changed()
        .flat_map(|html| observable::of_option(KickertoolData::from_html(html)))
        .tap(|data| println!("Parsed data: {:?}", data))
        .distinct_until_changed()
        .tap(|data| println!("Distinct data: {:?}", data))
        .share()
        .into_shared()
}

type Table1Observable = impl Observable<Item = Table, Err = ()> + Clone + SharedObservable;

impl Kickertool {
    pub fn new(url_html_observable: UrlHtmlObservable) -> Self {
        let kickertool_data_observable = get_kickertool_data_observable(url_html_observable);
        let mut s = Self {
            team_subscriptions: [None, None],
            standings_subscription: None,
            kickertool_data_observable,
        };
        s.standings_subscribe();
        s.team_subscribe(1);
        s.team_subscribe(2);

        s
    }

    fn standings_subscribe(&mut self) {
        Self::unsubscribe(&mut self.standings_subscription);

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

    fn get_table1_observable(&self) -> Table1Observable {
        self.kickertool_data_observable
            .clone()
            .filter_map(|data: KickertoolData| {
                data.tables.into_iter().find(|table| table.number == 1)
            })
    }

    fn team_subscribe(&mut self, number: usize) {
        self.team_unsubscribe(number);

        let sink = FileSink::new(format!("team{number}.txt"));

        let s = self
            .get_table1_observable()
            .map(move |data| match number {
                1 => data.team1,
                2 => data.team2,
                _ => unreachable!(),
            })
            .distinct_until_changed()
            .tap(move |team| println!("Team{number}: {team}"))
            .into_shared()
            .subscribe(move |standings| sink.sink(standings));

        let subscription = self.get_team_subscription_mut(number);
        *subscription = (Box::new(s) as Box<dyn SubscriptionLike>).into();
    }

    fn team_unsubscribe(&mut self, number: usize) {
        Self::unsubscribe(self.get_team_subscription_mut(number));
    }

    fn unsubscribe(subscription: &mut Option<Box<dyn SubscriptionLike>>) {
        if let Some(mut subscription) = subscription.take() {
            subscription.unsubscribe();
        }
    }

    fn get_team_subscription_mut(
        &mut self,
        number: usize,
    ) -> &mut Option<Box<dyn SubscriptionLike>> {
        &mut self.team_subscriptions[number - 1]
    }
}

impl Drop for Kickertool {
    fn drop(&mut self) {
        self.team_unsubscribe(1);
        self.team_unsubscribe(2);
        Self::unsubscribe(&mut self.standings_subscription);
    }
}
