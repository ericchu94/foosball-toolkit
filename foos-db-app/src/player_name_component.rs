use yew::prelude::*;

use crate::hooks::{use_player, use_rating};

#[derive(Properties, PartialEq)]
pub struct PlayerNameProps {
    pub id: i32,
    pub match_id: i32,
}

#[function_component]
pub fn PlayerNameComponent(props: &PlayerNameProps) -> Html {
    let player_id = props.id;
    let match_id = props.match_id;
    let player = use_player(player_id);
    let rating = use_rating(player_id, match_id);

    let name = if player.last_name.is_empty() {
        player.first_name
    } else {
        format!("{} {}", player.first_name, player.last_name)
    };

    let sign = if rating.after_rating < rating.before_rating {
        '-'
    } else {
        '+'
    };

    html! {
        <>
            <div>
                {name}
            </div>
            <small class="text-muted">{format!("{} {} {}", rating.before_rating, sign, rating.after_rating.abs_diff(rating.before_rating))}</small>
        </>
    }
}
