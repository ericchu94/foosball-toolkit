use yew::prelude::*;

use rxrust::prelude::*;
use rxrust::scheduler::LocalSpawner;

use crate::models::*;

use super::BASE_URL;

type N = impl FnMut(Vec<PlayerData>);
type O = impl Observable<Item = Vec<PlayerData>> + SubscribeNext<'static, N>;

fn get_observable(limit: i32) -> O {
    observable::from_future(
        reqwest::get(format!(
            "{BASE_URL}/player_data?limit={limit}"
        )),
        LocalSpawner {},
    )
    .flat_map(observable::from_iter)
    .flat_map(|a| observable::from_future(a.json::<Vec<PlayerData>>(), LocalSpawner {}))
    .flat_map(observable::from_iter)
}

#[hook]
pub fn use_player_datas(limit: i32) -> Vec<PlayerData> {
    let item = use_state(Vec::default);

    {
        let item = item.clone();
        use_effect_with_deps(
            move |&limit| {
                let mut s = get_observable(limit).subscribe(move |i| item.set(i));

                move || {
                    s.unsubscribe();
                }
            },
            limit,
        );
    }

    (*item).clone()
}
