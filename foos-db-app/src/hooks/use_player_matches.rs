use yew::prelude::*;

use rxrust::prelude::*;
use rxrust::scheduler::LocalSpawner;

use crate::models::*;

use super::BASE_URL;

type N = impl FnMut(Vec<PlayerMatch>);
type O = impl Observable<Item = Vec<PlayerMatch>> + SubscribeNext<'static, N>;

fn get_observable(match_id: i32) -> O {
    observable::from_future(
        reqwest::get(format!(
            "{BASE_URL}/player_match?match_id={match_id}"
        )),
        LocalSpawner {},
    )
    .flat_map(observable::from_iter)
    .flat_map(|a| observable::from_future(a.json::<Vec<PlayerMatch>>(), LocalSpawner {}))
    .flat_map(observable::from_iter)
}

#[hook]
pub fn use_player_matches(id: i32) -> Vec<PlayerMatch> {
    let item = use_state(Vec::new);

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
