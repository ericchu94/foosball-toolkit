use yew::prelude::*;

use rxrust::prelude::*;
use rxrust::scheduler::LocalSpawner;

use crate::models::*;

use super::BASE_URL;

type N = impl FnMut(Vec<Player>);
type O = impl Observable<Item = Vec<Player>> + SubscribeNext<'static, N>;

fn get_observable() -> O {
    observable::from_future(reqwest::get(format!("{BASE_URL}/player")), LocalSpawner {})
        .flat_map(observable::from_iter)
        .flat_map(|a| observable::from_future(a.json::<Vec<Player>>(), LocalSpawner {}))
        .flat_map(observable::from_iter)
}

#[hook]
pub fn use_players() -> Vec<Player> {
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
