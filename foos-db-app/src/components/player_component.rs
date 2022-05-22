use yew::prelude::*;

use crate::models::*;

#[derive(Properties, PartialEq)]
pub struct PlayerComponentProps {
    pub player_data: PlayerData,
    pub index: usize,
}

#[function_component]
pub fn PlayerComponent(props: &PlayerComponentProps) -> Html {
    let player_data = &props.player_data;

    let icon = match props.index {
        0 => Some("FFD700"),
        1 => Some("C0C0C0"),
        2 => Some("CD7F32"),
        _ => None,
    }.map(|color| {
        html! {
            <i class="bi-trophy-fill mx-1" style={format!("color: #{color}")}></i>
        }
    });

    html! {
        <>
            <button type="button" class="list-group-item list-group-item-action text-center">
                <span class="card-title h5">{for icon}{format!("{} {}", player_data.first_name, player_data.last_name)}</span><br /><small class="text-muted">{player_data.rating}</small>
            </button>
        </>
    }
}
