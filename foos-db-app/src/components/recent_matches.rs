use yew::prelude::*;
use crate::hooks::use_matches;
use super::MatchComponent;

#[function_component]
pub fn RecentMatches() -> Html {

    let matches = use_matches(5, 0);

    html! {
        <>
            <div class="card">
                <div class="card-header">
                {"Recent Matches"}
                </div>
                <div class="list-group list-group-flush">
                    {matches.iter().cloned().map(|r#match| {
                        html!{
                            <MatchComponent {r#match} />
                        }
                    }).collect::<Html>()}
                </div>
            </div>
        </>
    }
}
