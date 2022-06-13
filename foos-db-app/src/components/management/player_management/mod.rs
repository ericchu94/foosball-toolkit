mod player;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    foos_db_client::FoosDbClient,
    hooks::{use_foos_db_client, BASE_URL},
    models::{Player, PlayerWithTournamentCount},
};

use player::PlayerManagementPlayer;

async fn get_players(tournament_id: Option<i32>) -> Vec<Player> {
    let mut url = format!("{BASE_URL}/player");
    if let Some(id) = tournament_id {
        url = format!("{url}?tournament_id={id}");
    }

    reqwest::get(url).await.unwrap().json().await.unwrap()
}

#[derive(Properties, PartialEq, Eq)]
pub struct PlayerManagementProperties {
    pub tournament_id: i32,
}

fn retrieve_players(
    tournament_id: i32,
    tournament_players: UseStateHandle<Vec<PlayerWithTournamentCount>>,
    players: UseStateHandle<Vec<Player>>,
    client: FoosDbClient,
) {
    spawn_local(async move {
        let p = client.get_players().await.expect("get players failed");
        let tp = client
            .get_players_by_tournament_id(tournament_id)
            .await
            .expect("get players by tournament_id failed");
        tournament_players.set(tp);
        players.set(p);
    });
}

#[function_component]
pub fn PlayerManagement(props: &PlayerManagementProperties) -> Html {
    let tournament_id = props.tournament_id;
    let tournament_players = use_state(Vec::new);
    let players = use_state(Vec::new);
    let client = use_foos_db_client();
    {
        let players = players.clone();
        let tournament_players = tournament_players.clone();
        let client = client.clone();
        use_effect_with_deps(
            move |_| {
                retrieve_players(
                    tournament_id,
                    tournament_players.clone(),
                    players.clone(),
                    client,
                );

                || {}
            },
            (),
        );
    }

    let options = players.iter().map(|p| html! {
        <option value={p.id.to_string()}>{format!("{} {} ({})", p.first_name, p.last_name, p.id)}</option>
    }).collect::<Html>();

    let onchange = {
        let tournament_players = tournament_players.clone();
        Callback::from(move |_| {
            retrieve_players(
                tournament_id,
                tournament_players.clone(),
                players.clone(),
                client.clone(),
            );
        })
    };

    html! {
        <>
            <ul class="list-group">
                {tournament_players.iter().cloned().map(|p| {
                    let key = p.player.id;
                    html! {
                        <PlayerManagementPlayer {key} onchange={onchange.clone()} player={p} options={options.clone()} />
                    }
                }).collect::<Html>()}
            </ul>
        </>
    }
}
