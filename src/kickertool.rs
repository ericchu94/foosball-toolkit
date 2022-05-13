mod scraper;

use std::{sync::Arc, time::Duration};

use futures::executor::ThreadPool;
use headless_chrome::Tab;
use rxrust::prelude::*;

use self::scraper::KickertoolData;

type N = impl FnMut(KickertoolData) + Send + Sync + 'static;
type KickertoolDataObservable = impl SubscribeNext<'static, N>
    + Clone
    + Observable<Item = KickertoolData, Err = ()>
    + SharedObservable;

pub struct Kickertool {
    team_subscriptions: [Option<Box<dyn SubscriptionLike>>; 2],
    standings_subscription: Option<Box<dyn SubscriptionLike>>,
    kickertool_data_observable: KickertoolDataObservable,
}

fn get_kickertool_data_observable(
    tab: Arc<Tab>,
    scheduler: ThreadPool,
) -> KickertoolDataObservable {
    observable::interval(Duration::from_secs(1), scheduler)
        .flat_map(move |_| observable::of_option(get_html(tab.clone())))
        .flat_map(|html| observable::of_option(KickertoolData::from_html(html)))
        .tap(|data| println!("Parsed data: {:?}", data))
        .distinct_until_changed()
        .tap(|data| println!("Distinct data: {:?}", data))
        .share()
        .into_shared()
}

fn get_html(tab: Arc<Tab>) -> Option<String> {
    let remote_object = tab
        .evaluate("document.documentElement.outerHTML", false)
        .ok()?;

    let json = remote_object.value?;
    let str = json.as_str()?;

    Some(str.to_owned())
}

impl Kickertool {
    pub fn new(tab: Arc<Tab>) -> Self {
        let scheduler = ThreadPool::new().unwrap();
        let kickertool_data_observable = get_kickertool_data_observable(tab, scheduler);
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

        let s = self
            .kickertool_data_observable
            .clone()
            .subscribe(move |data| {
                let standings = data.standings;
                for line in &standings {
                    println!("{line}");
                }
                std::fs::write("standings.txt", standings.join("\n")).unwrap()
            });

        self.standings_subscription = (Box::new(s) as Box<dyn SubscriptionLike>).into();
    }

    fn team_subscribe(&mut self, number: usize) {
        self.team_unsubscribe(number);

        let s = self
            .kickertool_data_observable
            .clone()
            .flat_map(move |data| {
                observable::of_option(match number {
                    1 => data.team1,
                    2 => data.team2,
                    _ => unreachable!(),
                })
            })
            .distinct_until_changed()
            .into_shared()
            .subscribe(move |team| {
                println!("Team{number}: {team}");
                std::fs::write(format!("team{number}.txt"), team).unwrap();
            });

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
