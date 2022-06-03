use super::PlayerComponent;
use crate::hooks::use_player_datas;
use yew::prelude::*;

#[function_component]
pub fn TopPlayers() -> Html {
    let player_datas = use_player_datas(10, 0);

    html! {
        <>
            <div class="card my-3">
                <div class="card-header">
                {"Top Players"}
                </div>
                <div class="list-group list-group-flush list-group-numbered">
                    {player_datas.into_iter().enumerate().map(|(i, player_data)| {
                        html!{
                            <PlayerComponent index={i} {player_data} />
                        }
                    }).collect::<Html>()}
                </div>
            </div>
        </>
    }
}
