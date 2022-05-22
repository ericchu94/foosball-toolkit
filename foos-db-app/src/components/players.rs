use super::*;
use crate::hooks::get_player_datas_observable;
use crate::models::*;
use rxrust::prelude::*;
use yew::prelude::*;

#[function_component]
pub fn Players() -> Html {
    let limit = 10;
    let player_datas = use_state(Vec::<PlayerData>::new);

    let offset = use_state(|| 0);

    let onclick = {
        let offset = offset.clone();
        Callback::from(move |_| {
            offset.set(*offset + 10);
        })
    };

    {
        let player_datas = player_datas.clone();
        use_effect_with_deps(
            move |&offset| {
                let mut subscription = get_player_datas_observable(limit, offset).subscribe(
                    move |mut new: Vec<PlayerData>| {
                        let mut v = (*player_datas).clone();
                        v.append(&mut new);
                        player_datas.set(v);
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
                {"Players"}
                </div>
                <div class="list-group list-group-flush list-group-numbered">
                    {player_datas.iter().cloned().enumerate().map(|(index, player_data)| {
                        let key = player_data.player_id;
                        html!{
                            <PlayerComponent {key} {index} {player_data} />
                        }
                    }).collect::<Html>()}
                    <style>{r#".more-btn::before { content: none !important }"#}</style>
                    <button type="button" class="more-btn text-center list-group-item list-group-item-action" aria-current="true" {onclick}>{"More ..."}</button>
                </div>
            </div>
        </>
    }
}
