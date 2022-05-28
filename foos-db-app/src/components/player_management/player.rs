use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::models::Player;

use crate::hooks::BASE_URL;

#[derive(Properties, PartialEq)]
pub struct PlayerProps {
    pub player: Player,
    pub options: Html,
    pub onchange: Callback<()>,
}

fn create_input_callback(handle: &UseStateHandle<String>) -> Callback<InputEvent> {
    let handle = handle.clone();
    Callback::from(move |e: InputEvent| {
        let target = e.target_unchecked_into::<HtmlInputElement>();
        handle.set(target.value());
    })
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

    let first_name = use_state(|| p.first_name.clone());
    let last_name = use_state(|| p.last_name.clone());
    let to = use_state(String::new);

    let first_name_on_input = create_input_callback(&first_name);
    let last_name_on_input = create_input_callback(&last_name);
    let merge_on_input = create_input_callback(&to);

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
            <select class="mx-1" type="select" oninput={merge_on_input}>
                <option value="" selected={true}>{"Merge into"}</option>
                {options.clone()}
            </select>
            <button class="mx-1" disabled={to.is_empty()} onclick={on_merge}>{"Merge"}</button>
        </li>
    }
}
