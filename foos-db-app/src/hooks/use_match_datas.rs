use yew::prelude::*;

use rxrust::prelude::*;
use rxrust::scheduler::LocalSpawner;

use crate::models::*;

use super::BASE_URL;

type O<N: FnMut(Vec<MatchData>) + 'static> = impl Observable<Item = Vec<MatchData>> + SubscribeNext<'static, N>;

pub fn get_match_datas_observable<N: FnMut(Vec<MatchData>) + 'static>(limit: i32, offset: i32) -> O<N> {
    observable::from_future(
        reqwest::get(format!(
            "{BASE_URL}/match_data?limit={limit}&offset={offset}"
        )),
        LocalSpawner {},
    )
    .flat_map(observable::from_iter)
    .flat_map(|a| observable::from_future(a.json::<Vec<MatchData>>(), LocalSpawner {}))
    .flat_map(observable::from_iter)
}

#[hook]
pub fn use_match_datas(limit: i32, offset: i32) -> Vec<MatchData> {
    let item = use_state(Vec::default);

    {
        let item = item.clone();
        use_effect_with_deps(
            move |&(limit, offset)| {
                let mut s = get_match_datas_observable(limit, offset).subscribe(move |i| item.set(i));

                move || {
                    s.unsubscribe();
                }
            },
            (limit, offset),
        );
    }

    (*item).clone()
}
