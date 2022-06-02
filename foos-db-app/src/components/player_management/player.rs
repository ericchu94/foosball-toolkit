use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::models::Player;

use crate::hooks::{BASE_URL, use_input};

#[derive(Properties, PartialEq)]
pub struct PlayerProps {
    pub player: Player,
    pub options: Html,
    pub onchange: Callback<()>,
}

#[derive(Serialize)]
struct Merge {
    from: i32,
    to: i32,
}

#[function_component]
pub fn PlayerManagementPlayer(props: &PlayerProps) -> Html {
    let p = &props.player;
    let options = &props.options;
    let onchange = props.onchange.clone();

    let (first_name, first_name_on_input) = use_input(|| p.first_name.clone());
    let (last_name, last_name_on_input) = use_input(|| p.last_name.clone());
    let (to, to_on_input) = use_input(String::new);

    let on_rename = {
        let first_name = first_name.clone();
        let last_name = last_name.clone();
        let player = p.clone();
        let onchange = onchange.clone();
        Callback::from(move |_| {
            let player = Player {
                first_name: (*first_name).clone(),
                last_name: (*last_name).clone(),
                ..player.clone()
            };

            let onchange = onchange.clone();

            spawn_local(async move {
                reqwest::Client::new()
                    .put(format!("{BASE_URL}/player/{}", player.id))
                    .json(&player)
                    .send()
                    .await
                    .unwrap();
                onchange.clone().emit(());
            });
        })
    };

    let on_merge = {
        let to = to.clone();
        let from = p.id;
        Callback::from(move |_| {
            let to = to.parse::<i32>().unwrap();
            let merge = Merge { to, from };

            let onchange = onchange.clone();

            spawn_local(async move {
                reqwest::Client::new()
                    .post(format!("{BASE_URL}/player/merge"))
                    .json(&merge)
                    .send()
                    .await
                    .unwrap();
                onchange.clone().emit(());
            });
        })
    };

    html! {
        <li class="list-group-item">
            {p.id}
            <input class="mx-1" type="text" value={(*first_name).clone()} placeholder="First Name" oninput={first_name_on_input} />
            <input class="mx-1" type="text" value={(*last_name).clone()} placeholder="Last Name" oninput={last_name_on_input} />
            <button class="mx-1" onclick={on_rename}>{"Rename"}</button>
            <select class="mx-1" type="select" oninput={to_on_input}>
                <option value="" selected={true}>{"Merge into"}</option>
                {options.clone()}
            </select>
            <button class="mx-1" disabled={to.is_empty()} onclick={on_merge}>{"Merge"}</button>
        </li>
    }
}
