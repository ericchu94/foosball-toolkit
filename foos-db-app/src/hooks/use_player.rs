use yew::prelude::*;

use rxrust::prelude::*;
use rxrust::scheduler::LocalSpawner;

use crate::models::*;

type N = impl FnMut(Player);
type O = impl Observable<Item = Player> + SubscribeNext<'static, N>;

fn get_observable(id: i32) -> O {
    observable::from_future(
        reqwest::get(format!(
            "http://192.168.2.12:8888/player/{id}"
        )),
        LocalSpawner {},
    )
    .flat_map(observable::from_iter)
    .flat_map(|a| observable::from_future(a.json::<Player>(), LocalSpawner {}))
    .flat_map(observable::from_iter)
}

#[hook]
pub fn use_player(id: i32) -> Player {
    let item = use_state(Player::default);

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
