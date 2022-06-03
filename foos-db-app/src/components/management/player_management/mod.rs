mod player;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{hooks::BASE_URL, models::Player};

use player::PlayerManagementPlayer;

async fn get_players(tournament_id: Option<i32>) -> Vec<Player> {
    let mut url = format!("{BASE_URL}/player");
    if let Some(id) = tournament_id {
        url = format!("{url}?tournament_id={id}");
    }

    reqwest::get(url).await.unwrap().json().await.unwrap()
}

#[derive(Properties, PartialEq)]
pub struct PlayerManagementProperties {
    pub tournament_id: i32,
}

fn retrieve_players(
    tournament_id: i32,
    tournament_players: UseStateHandle<Vec<Player>>,
    players: UseStateHandle<Vec<Player>>,
) {
    spawn_local(async move {
        let p = get_players(None).await;
        let tp = get_players(Some(tournament_id)).await;
        tournament_players.set(tp);
        players.set(p);
    });
}

#[function_component]
pub fn PlayerManagement(props: &PlayerManagementProperties) -> Html {
    let tournament_id = props.tournament_id.clone();
    let tournament_players = use_state(Vec::new);
    let players = use_state(Vec::new);
    {
        let players = players.clone();
        let tournament_players = tournament_players.clone();
        use_effect_with_deps(
            move |_| {
                retrieve_players(
                    tournament_id.clone(),
                    tournament_players.clone(),
                    players.clone(),
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
                tournament_id.clone(),
                tournament_players.clone(),
                players.clone(),
            );
        })
    };

    html! {
        <>
            <ul class="list-group">
                {tournament_players.iter().cloned().map(|p| {
                    let key = p.id;
                    html! {
                        <PlayerManagementPlayer {key} onchange={onchange.clone()} player={p} options={options.clone()} />
                    }
                }).collect::<Html>()}
            </ul>
        </>
    }
}
