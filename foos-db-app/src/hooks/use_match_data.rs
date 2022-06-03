use yew::prelude::*;

use rxrust::prelude::*;
use rxrust::scheduler::LocalSpawner;

use crate::models::*;

use super::BASE_URL;

type N = impl FnMut(MatchData);
type O = impl Observable<Item = MatchData> + SubscribeNext<'static, N>;

fn get_observable(id: i32) -> O {
    observable::from_future(
        reqwest::get(format!("{BASE_URL}/match_data/{id}")),
        LocalSpawner {},
    )
    .flat_map(observable::from_iter)
    .flat_map(|a| observable::from_future(a.json::<MatchData>(), LocalSpawner {}))
    .flat_map(observable::from_iter)
}

#[hook]
pub fn use_match_data(id: i32) -> MatchData {
    let item = use_state(MatchData::default);

    {
        let item = item.clone();
        use_effect_with_deps(
            move |&id| {
                let mut s = get_observable(id).subscribe(move |i| item.set(i));

                move || {
                    s.unsubscribe();
                }
            },
            id,
        );
    }

    (*item).clone()
}
