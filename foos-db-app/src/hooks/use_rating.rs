use yew::prelude::*;

use rxrust::prelude::*;
use rxrust::scheduler::LocalSpawner;

use crate::models::*;

use super::BASE_URL;

type N = impl FnMut(Rating);
type O = impl Observable<Item = Rating> + SubscribeNext<'static, N>;

fn get_observable(player_id: i32, match_id: i32) -> O {
    observable::from_future(
        reqwest::get(format!("{BASE_URL}/rating/{player_id}?match_id={match_id}")),
        LocalSpawner {},
    )
    .flat_map(observable::from_iter)
    .flat_map(|res| observable::from_future(res.json::<Rating>(), LocalSpawner {}))
    .flat_map(observable::from_iter)
}

#[hook]
pub fn use_rating(player_id: i32, match_id: i32) -> Rating {
    let item = use_state(Rating::default);

    {
        let item = item.clone();
        use_effect_with_deps(
            move |&(player_id, match_id)| {
                let mut s = get_observable(player_id, match_id).subscribe(move |i| item.set(i));

                move || {
                    s.unsubscribe();
                }
            },
            (player_id, match_id),
        );
    }

    (*item).clone()
}
