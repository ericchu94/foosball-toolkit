use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::models::Player;

use crate::hooks::BASE_URL;

#[derive(Properties, PartialEq)]
pub struct PlayerProps {
    pub player: Player,
    pub options: Html,
}

fn create_input_callback(handle: &UseStateHandle<String>) -> Callback<InputEvent> {
    let handle = handle.clone();
    Callback::from(move |e: InputEvent| {
        let target = e.target_unchecked_into::<HtmlInputElement>();
        handle.set(target.value());
    })
}

#[function_component]
pub fn PlayerManagementPlayer(props: &PlayerProps) -> Html {
    let p = &props.player;
    let options = &props.options;

    let first_name = use_state(|| p.first_name.clone());
    let last_name = use_state(|| p.last_name.clone());

    let first_name_on_input = create_input_callback(&first_name);
    let last_name_on_input = create_input_callback(&last_name);

    let on_rename = {
        let first_name = first_name.clone();
        let last_name = last_name.clone();
        let player = p.clone();
        Callback::from(move |_| {
            let player = Player {
                first_name: (*first_name).clone(),
                last_name: (*last_name).clone(),
                ..player.clone()
            };

            spawn_local(async move {
                reqwest::Client::new()
                    .put(format!("{BASE_URL}/player/{}", player.id))
                    .json(&player)
                    .send()
                    .await.unwrap();
            });
        })
    };

    html! {
        <li>
            {p.id}
            <input class="mx-1" type="text" value={(*first_name).clone()} placeholder="First Name" oninput={first_name_on_input} />
            <input class="mx-1" type="text" value={(*last_name).clone()} placeholder="Last Name" oninput={last_name_on_input} />
            <button class="mx-1" onclick={on_rename}>{"Rename"}</button>
            <select class="mx-1" type="select">
                <option selected={true}>{"Merge into"}</option>
                {options.clone()}
            </select>
            <button class="mx-1">{"Merge"}</button>
            <button class="mx-1">{"Delete"}</button>
        </li>
    }
}
