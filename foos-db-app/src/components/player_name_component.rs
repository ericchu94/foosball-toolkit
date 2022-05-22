use yew::prelude::*;

use crate::models::MatchDataPlayer;

#[derive(Properties, PartialEq)]
pub struct PlayerNameProps {
    pub player: MatchDataPlayer,
}

#[function_component]
pub fn PlayerNameComponent(props: &PlayerNameProps) -> Html {
    let player = &props.player;

    let name = if player.last_name.is_empty() {
        player.first_name.clone()
    } else {
        format!("{} {}", player.first_name, player.last_name)
    };

    let sign = if player.after_rating < player.before_rating {
        '-'
    } else {
        '+'
    };

    html! {
        <>
            <div>
                {name}
            </div>
            <small class="text-muted">{format!("{} {} {}", player.before_rating, sign, player.after_rating.abs_diff(player.before_rating))}</small>
        </>
    }
}
