use super::MatchComponent;
use crate::hooks::use_match_datas;
use yew::prelude::*;

#[function_component]
pub fn RecentMatches() -> Html {
    let match_datas = use_match_datas(5, 0);

    html! {
        <>
            <div class="card my-3">
                <div class="card-header">
                {"Recent Matches"}
                </div>
                <div class="list-group list-group-flush">
                    {match_datas.iter().cloned().map(|match_data| {
                        html!{
                            <MatchComponent {match_data} />
                        }
                    }).collect::<Html>()}
                </div>
            </div>
        </>
    }
}
