use yew::prelude::*;

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct StandingsProps {
    pub standings: Vec<String>,
}

#[function_component]
pub fn Standings(props: &StandingsProps) -> Html {
    let standings = &props.standings;

    html! {
        <>
            <style>{"
            .standings {
                padding: 10px;
            }
            "}</style>
            <div class="standings">
                <h1>{"Standings"}</h1>
                <hr />
                <ol>
                    {standings.iter().map(|line| html! {
                        <li>{line.split_whitespace().nth(1).unwrap()}</li>
                    }).collect::<Html>()}
                </ol>
            </div>
        </>
    }
}
