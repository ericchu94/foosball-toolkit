use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    components::PlayerManagement,
    hooks::{use_foos_db_client, use_input},
    models::Tournament,
};

#[derive(Properties, PartialEq)]
pub struct TournamentManagementProperties {
    pub id: i32,
}

#[function_component]
pub fn TournamentManagement(props: &TournamentManagementProperties) -> Html {
    let id = props.id;

    let tournament = use_state(Tournament::default);
    let (tournament_name, tournament_name_on_input) = use_input(Default::default);

    {
        let tournament = tournament.clone();
        let tournament_name = tournament_name.clone();
        let client = use_foos_db_client();
        use_effect_with_deps(
            move |&id| {
                spawn_local(async move {
                    let t = client.tournament(id).await.unwrap();
                    tournament_name.set(t.name.clone());
                    tournament.set(t);
                });

                || {}
            },
            id,
        );
    }

    let on_rename = {
        let client = use_foos_db_client();
        let tournament_name = tournament_name.clone();
        let tournament = tournament.clone();
        Callback::from(move |_| {
            let client = client.clone();
            let tournament_name = tournament_name.clone();
            let tournament = tournament.clone();
            spawn_local(async move {
                let t = Tournament {
                    name: (*tournament_name).clone(),
                    ..(*tournament).clone()
                };
                client.put_tournament(&t).await.unwrap();
                tournament.set(t);
            })
        })
    };

    html! {
        <>
            <div class="row">
                <div class="col input-group mb-1">
                    <input class="form-control" type="text" placeholder="Tournament Name" value={(*tournament_name).clone()} oninput={tournament_name_on_input} />
                    <button class="btn" onclick={on_rename} disabled={(*tournament_name) == tournament.name}>{"Rename"}</button>
                </div>
            </div>
            <div class="row">
                <PlayerManagement tournament_id={id} />
            </div>
            <div class="row">
                <a href="#">{"Download raw"}</a>
            </div>
        </>
    }
}
