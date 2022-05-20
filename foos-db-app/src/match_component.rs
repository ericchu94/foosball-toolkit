
use std::collections::HashMap;

use time::macros::format_description;
use yew::prelude::*;

use crate::models::*;
use crate::hooks::{use_tournament, use_player_matches};
use crate::player_name_component::PlayerNameComponent;

#[derive(Properties, PartialEq)]
pub struct MatchProps {
    pub r#match: Match,
}

#[function_component]
pub fn MatchComponent(props: &MatchProps) -> Html {
    let m = &props.r#match;

    let tournament = use_tournament(m.tournament_id.unwrap());

    let player_matches = use_player_matches(m.id);

    let class_tie = "bi-emoji-neutral text-warning";
    let class_win = "bi-emoji-smile text-success";
    let class_lose = "bi-emoji-frown text-danger";
    let (classes1, classes2) = match m.winner {
        Winner::Draw => (class_tie, class_tie),
        Winner::Team1 => (class_win, class_lose),
        Winner::Team2 => (class_lose, class_win),
        _ => unreachable!(),
    };

    let mut map = HashMap::new();
    map.insert(Team::Team1, vec![]);
    map.insert(Team::Team2, vec![]);
    let map = player_matches.into_iter().fold(map, |mut acc, pm| {
        acc.entry(pm.team).or_insert(vec![]).push(pm.player_id);
        acc
    });

    let format = format_description!("[year]-[month]-[day] [hour]:[minute]");

    html! {
        <>
            <button type="button" class="list-group-item list-group-item-action" aria-current="true">
                <div class="text-muted fs-6">
                    <div class="position-absolute start-0 ms-2">
                        <small>{&tournament.name}</small>
                    </div>
                    <div class="position-absolute end-0 me-2">
                        <small>{m.timestamp.format(format).unwrap()}</small>
                    </div>
                </div>
                <div class="row">
                    <div class="col-sm text-sm-end text-center align-self-top">
                        <i class={classes!(classes1)}></i>
                        {map[&Team::Team1].iter().map(|id| html! { <PlayerNameComponent {id} /> }).collect::<Html>()}
                    </div>
                    <div class="col-sm-1 text-center align-self-center">
                        <i class="bi-x px-1"></i>
                    </div>
                    <div class="col-sm text-sm-start text-center align-self-top">
                        <i class={classes!(classes2)}></i>
                        {map[&Team::Team2].iter().map(|id| html! { <PlayerNameComponent {id} /> }).collect::<Html>()}
                    </div>
                </div>
            </button>
        </>
    }
}
