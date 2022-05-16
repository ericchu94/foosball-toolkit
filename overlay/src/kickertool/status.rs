use kickertool_data::Match;
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct StatusProps {
    pub r#match: Match,
}

#[function_component]
pub fn Status(props: &StatusProps) -> Html {
    let r#match = &props.r#match;

    html! {
        <>
            <style>{"
            .status {
                display: grid;
                height: 100%;
                grid-template-columns: 40% auto 40%;
            }
            .status > div {
                align-self: center;
                font-size: 64pt;
                padding: 50px;
            }
            .status-team1 {
                grid-column: 1;
                justify-self: left;
            }
            .status-team2 {
                grid-column: 3;
                justify-self: right;
            }
            "}</style>
            <div class="status">
                <div class="status-team1">{&r#match.team1}</div>
                <div class="status-team2">{&r#match.team2}</div>
            </div>
        </>
    }
}
