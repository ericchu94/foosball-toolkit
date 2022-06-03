use yew::prelude::*;

use rxrust::prelude::*;
use rxrust::scheduler::LocalSpawner;

use crate::models::*;

use super::BASE_URL;

type O<N: FnMut(Vec<PlayerData>) + 'static> =
    impl Observable<Item = Vec<PlayerData>> + SubscribeNext<'static, N>;

pub fn get_player_datas_observable<N: FnMut(Vec<PlayerData>) + 'static>(
    limit: i32,
    offset: i32,
) -> O<N> {
    observable::from_future(
        reqwest::get(format!(
            "{BASE_URL}/player_data?limit={limit}&offset={offset}"
        )),
        LocalSpawner {},
    )
    .flat_map(observable::from_iter)
    .flat_map(|a| observable::from_future(a.json::<Vec<PlayerData>>(), LocalSpawner {}))
    .flat_map(observable::from_iter)
}

#[hook]
pub fn use_player_datas(limit: i32, offset: i32) -> Vec<PlayerData> {
    let item = use_state(Vec::default);

    {
        let item = item.clone();
        use_effect_with_deps(
            move |&(limit, offset)| {
                let mut s =
                    get_player_datas_observable(limit, offset).subscribe(move |i| item.set(i));

                move || {
                    s.unsubscribe();
                }
            },
            (limit, offset),
        );
    }

    (*item).clone()
}
