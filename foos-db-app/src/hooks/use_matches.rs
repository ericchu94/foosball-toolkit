use yew::prelude::*;

use rxrust::prelude::*;
use rxrust::scheduler::LocalSpawner;

use crate::models::*;

use super::BASE_URL;

type N = impl FnMut(Vec<Match>);
type O = impl Observable<Item = Vec<Match>> + SubscribeNext<'static, N>;

fn get_observable() -> O {
    observable::from_future(
        reqwest::get(format!("{BASE_URL}/match?limit=5")),
        LocalSpawner {},
    )
    .flat_map(observable::from_iter)
    .flat_map(|response| observable::from_future(response.json::<Vec<Match>>(), LocalSpawner {}))
    .flat_map(observable::from_iter)
}

#[hook]
pub fn use_matches() -> Vec<Match> {
    let item = use_state(Vec::default);

    {
        let item = item.clone();
        use_effect_with_deps(
            move |_| {
                let mut s = get_observable().subscribe(move |i| item.set(i));

                move || {
                    s.unsubscribe();
                }
            },
            (),
        );
    }

    (*item).clone()
}
