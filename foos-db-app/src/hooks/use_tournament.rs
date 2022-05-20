use yew::prelude::*;

use rxrust::prelude::*;
use rxrust::scheduler::LocalSpawner;

use crate::models::*;

use super::BASE_URL;

type TournamentNext = impl FnMut(Tournament);
type TournamentObservable =
    impl Observable<Item = Tournament> + SubscribeNext<'static, TournamentNext>;

fn get_tournament_observable(tournament_id: i32) -> TournamentObservable {
    observable::from_future(
        reqwest::get(format!("{BASE_URL}/tournament/{tournament_id}")),
        LocalSpawner {},
    )
    .flat_map(observable::from_iter)
    .flat_map(|a| observable::from_future(a.json::<Tournament>(), LocalSpawner {}))
    .flat_map(observable::from_iter)
}

#[hook]
pub fn use_tournament(id: i32) -> Tournament {
    let tournament = use_state(Tournament::default);

    {
        let tournament = tournament.clone();
        use_effect_with_deps(
            move |&id| {
                let mut s = get_tournament_observable(id).subscribe(move |t| tournament.set(t));

                move || {
                    s.unsubscribe();
                }
            },
            id,
        );
    }

    (*tournament).clone()
}
