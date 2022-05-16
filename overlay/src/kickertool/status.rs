use kickertool_data::Match;
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct StatusProps {
    pub r#match: Match,
    pub font_size: u16,
}

#[function_component]
pub fn Status(props: &StatusProps) -> Html {
    let r#match = &props.r#match;
    let font_size = props.font_size;

    html! {
        <>
            <style>{format!("
            .status {{
                display: grid;
                height: 100%;
                grid-template-columns: 50% auto 50%;
            }}
            .status > div {{
                align-self: center;
                font-size: {font_size}pt;
                padding: 50px;
            }}
            .status-team1 {{
                grid-column: 1;
                justify-self: left;
            }}
            .status-team2 {{
                grid-column: 3;
                justify-self: right;
            }}
            ")}</style>
            <div class="status">
                <div class="status-team1">{&r#match.team1}</div>
                <div class="status-team2">{&r#match.team2}</div>
            </div>
        </>
    }
}
