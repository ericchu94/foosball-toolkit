use super::MatchComponent;
use crate::foos_db_client::FoosDbClient;
use crate::hooks::get_match_datas_observable;
use crate::models::*;
use rxrust::prelude::*;
use yew::prelude::*;

#[function_component]
pub fn Matches() -> Html {
    let limit = 10;
    let match_datas = use_state(Vec::<MatchData>::new);

    let offset = use_state(|| 0);

    let onclick = {
        let offset = offset.clone();
        Callback::from(move |_| {
            offset.set(*offset + 10);
        })
    };

    let client = use_context::<FoosDbClient>().unwrap();

    {
        let match_datas = match_datas.clone();
        use_effect_with_deps(
            move |&offset| {
                let mut subscription = get_match_datas_observable(client, limit, offset).subscribe(
                    move |mut new: Vec<MatchData>| {
                        let mut v = (*match_datas).clone();
                        v.append(&mut new);
                        match_datas.set(v);
                    },
                );
                move || {
                    subscription.unsubscribe();
                }
            },
            *offset,
        );
    }

    html! {
        <>
            <div class="card my-3">
                <div class="card-header">
                {"Matches"}
                </div>
                <div class="list-group list-group-flush">
                    {match_datas.iter().cloned().map(|match_data| {
                        let key = match_data.id;
                        html!{
                            <MatchComponent {key} {match_data} />
                        }
                    }).collect::<Html>()}
                    <button type="button" class="text-center list-group-item list-group-item-action" aria-current="true" {onclick}>{"More ..."}</button>
                </div>
            </div>
        </>
    }
}
