use yew::prelude::*;

use rxrust::prelude::*;
use rxrust::scheduler::LocalSpawner;

use crate::{foos_db_client::FoosDbClient, models::*};

use super::{use_foos_db_client};

type TournamentNext = impl FnMut(Tournament);
type TournamentObservable =
    impl Observable<Item = Tournament> + SubscribeNext<'static, TournamentNext>;

fn get_tournament_observable(client: FoosDbClient, tournament_id: i32) -> TournamentObservable {
    observable::from_future(
        async move { client.tournament(tournament_id).await },
        LocalSpawner {},
    )
    .flat_map(observable::from_iter)
}

#[hook]
pub fn use_tournament(id: i32) -> Tournament {
    let client = use_foos_db_client();
    let tournament = use_state(Tournament::default);

    {
        let tournament = tournament.clone();
        use_effect_with_deps(
            move |&id| {
                let mut s =
                    get_tournament_observable(client, id).subscribe(move |t| tournament.set(t));

                move || {
                    s.unsubscribe();
                }
            },
            id,
        );
    }

    (*tournament).clone()
}
