use kickertool_data::Match;
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct NextProps {
    pub next_matches: Vec<Match>,
}

#[function_component]
pub fn Next(props: &NextProps) -> Html {
    let next_matches = &props.next_matches;

    html! {
        <>
            <style>{"
            .next {
                padding: 25px;
            }
            .next li {
                font-size: 36px;
            }
            "}</style>
            <div class="next">
                <h1>{"Next Matches"}</h1>
                <hr />
                <ol>
                    {next_matches.iter().map(|r#match| html! {
                        <li>{format!("{} vs {}", r#match.team1, r#match.team2)}</li>
                    }).collect::<Html>()}
                </ol>
            </div>
        </>
    }
}
