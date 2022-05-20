use yew::prelude::*;

use rxrust::prelude::*;
use rxrust::scheduler::LocalSpawner;

use crate::models::*;

use super::BASE_URL;

type O<N: FnMut(Vec<Match>) + 'static> = impl Observable<Item = Vec<Match>> + SubscribeNext<'static, N>;

pub fn get_matches_observable<N: FnMut(Vec<Match>) + 'static>(limit: i32, offset: i32) -> O<N> {
    observable::from_future(
        reqwest::get(format!("{BASE_URL}/match?limit={limit}&offset={offset}")),
        LocalSpawner {},
    )
    .flat_map(observable::from_iter)
    .flat_map(|response| observable::from_future(response.json::<Vec<Match>>(), LocalSpawner {}))
    .flat_map(observable::from_iter)
}

#[hook]
pub fn use_matches(limit: i32, offset: i32) -> Vec<Match> {
    let item = use_state(Vec::default);

    {
        let item = item.clone();
        use_effect_with_deps(
            move |_| {
                let mut s = get_matches_observable(limit, offset).subscribe(move |i| item.set(i));

                move || {
                    s.unsubscribe();
                }
            },
            (),
        );
    }

    (*item).clone()
}
