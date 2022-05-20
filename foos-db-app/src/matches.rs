use crate::hooks::use_matches::get_matches_observable;
use crate::match_component::MatchComponent;
use crate::models::Match;
use rxrust::prelude::*;
use yew::prelude::*;

#[function_component]
pub fn Matches() -> Html {
    let limit = 10;
    let matches = use_state(Vec::<Match>::new);

    let offset = use_state(|| 0);

    let onclick = {
        let offset = offset.clone();
        Callback::from(move |_| {
            offset.set(*offset + 10);
        })
    };

    {
        let matches = matches.clone();
        use_effect_with_deps(
            move |&offset| {
                let mut subscription =
                    get_matches_observable(limit, offset).subscribe(move |mut new_matches: Vec<Match>| {
                        let mut v = (*matches).clone();
                        v.append(&mut new_matches);
                        matches.set(v);
                    });
                move || {
                    subscription.unsubscribe();
                }
            },
            *offset,
        );
    }

    html! {
        <>
            <div class="card">
                <div class="card-header">
                {"Matches"}
                </div>
                <div class="list-group list-group-flush">
                    {matches.iter().cloned().map(|r#match| {
                        html!{
                            <MatchComponent {r#match} />
                        }
                    }).collect::<Html>()}
                    <button type="button" class="text-center list-group-item list-group-item-action" aria-current="true" {onclick}>{"More ..."}</button>
                </div>
            </div>
        </>
    }
}
