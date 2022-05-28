mod player;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{hooks::BASE_URL, models::Player};

use player::PlayerManagementPlayer;

async fn get_players() -> Vec<Player> {
    reqwest::get(format!("{BASE_URL}/player"))
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

#[function_component]
pub fn PlayerManagement() -> Html {
    let players = use_state(Vec::new);
    {
        let players = players.clone();
        use_effect_with_deps(
            |_| {
                spawn_local(async move {
                    let p = get_players().await;
                    players.set(p);
                });

                || {}
            },
            (),
        );
    }

    let options = players.iter().map(|p| html! {
        <option value={p.id.to_string()}>{format!("{} {} {}", p.id, p.first_name, p.last_name)}</option>
    }).collect::<Html>();

    let onchange = {
        let players = players.clone();
        Callback::from(move |_| {
            let players = players.clone();
            spawn_local(async move {
                let p = get_players().await;
                players.set(p);
            });
        })
    };

    html! {
        <>
            <ul class="list-group">
                {players.iter().cloned().map(|p| {
                    let key = p.id;
                    html! {
                        <PlayerManagementPlayer {key} onchange={onchange.clone()} player={p} options={options.clone()} />
                    }
                }).collect::<Html>()}
            </ul>
        </>
    }
}
