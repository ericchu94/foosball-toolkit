use yew::prelude::*;

use crate::hooks::use_player;

#[derive(Properties, PartialEq)]
pub struct PlayerNameProps {
    pub id: i32,
}

#[function_component]
pub fn PlayerNameComponent(props: &PlayerNameProps) -> Html {
    let player = use_player(props.id);

    let name = if player.last_name.is_empty() {
        player.first_name
    } else {
        format!("{} {}", player.first_name, player.last_name)
    };

    html! {
        <>
            <div>
                {name}
            </div>
        </>
    }
}
