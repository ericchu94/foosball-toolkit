use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::hooks::use_foos_db_client;

use super::super::app::Route;

#[function_component]
pub fn Management() -> Html {
    let client = use_foos_db_client();
    let tournaments = use_state(Vec::new);

    {
        let tournaments = tournaments.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    tournaments.set(client.tournaments().await.unwrap());
                });

                || {}
            },
            (),
        );
    }

    html! {
        <>
            {"Management"}
            <div class="list-group">
            {tournaments.iter().map(|tournament| html! {
                <>
                    <Link<Route> to={Route::TournamentManagement { id: tournament.id }} classes="list-group-item">
                        {&tournament.name}
                    </Link<Route>>
                </>
            }).collect::<Html>()}
            </div>
        </>
    }
}
