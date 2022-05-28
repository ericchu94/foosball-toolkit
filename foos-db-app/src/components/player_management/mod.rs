mod player;

use yew::prelude::*;

use crate::hooks::use_players;

use player::PlayerManagementPlayer;

#[function_component]
pub fn PlayerManagement() -> Html {
    let players = use_players();

    let options = players.iter().map(|p| html! {
        <option value={p.id.to_string()}>{format!("{} {} {}", p.id, p.first_name, p.last_name)}</option>
    }).collect::<Html>();

    html! {
        <>
            {"PlayerManagement"}
            <ul>
                {players.iter().cloned().map(|p| html! {
                    <PlayerManagementPlayer player={p} options={options.clone()} />
                }).collect::<Html>()}
            </ul>
        </>
    }
}
