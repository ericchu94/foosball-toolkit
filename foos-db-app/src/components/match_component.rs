use time::macros::format_description;
use yew::prelude::*;

use super::PlayerNameComponent;
use crate::models::*;

#[derive(Properties, PartialEq)]
pub struct MatchProps {
    pub match_data: MatchData,
}

#[function_component]
pub fn MatchComponent(props: &MatchProps) -> Html {
    let match_data = &props.match_data;

    let class_tie = "bi-emoji-neutral text-warning";
    let class_win = "bi-emoji-smile text-success";
    let class_lose = "bi-emoji-frown text-danger";
    let (classes1, classes2) = match match_data.winner {
        Winner::Draw => (class_tie, class_tie),
        Winner::Team1 => (class_win, class_lose),
        Winner::Team2 => (class_lose, class_win),
        _ => unreachable!(),
    };

    let format = format_description!("[year]-[month]-[day] [hour]:[minute]");

    html! {
        <>
            <button type="button" class="list-group-item list-group-item-action" aria-current="true">
                <div class="text-muted fs-6">
                    <div class="position-absolute start-0 ms-2">
                        <small>{&match_data.tournament_name}</small>
                    </div>
                    <div class="position-absolute end-0 me-2">
                        <small>{match_data.timestamp.format(format).unwrap()}</small>
                    </div>
                </div>
                <div class="row">
                    <div class="col-sm text-sm-end text-center align-self-top">
                        <i class={classes!(classes1)}></i>
                        {match_data.team1.iter().cloned().map(|player| html! { <>
                            <PlayerNameComponent {player} />
                        </> }).collect::<Html>()}
                    </div>
                    <div class="col-sm-1 text-center align-self-center">
                        <i class="bi-x px-1"></i>
                    </div>
                    <div class="col-sm text-sm-start text-center align-self-top">
                        <i class={classes!(classes2)}></i>
                        {match_data.team2.iter().cloned().map(|player| html! { <>
                            <PlayerNameComponent {player} />
                        </> }).collect::<Html>()}
                    </div>
                </div>
            </button>
        </>
    }
}
