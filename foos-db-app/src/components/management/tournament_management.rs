use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{hooks::use_foos_db_client, models::Tournament};

#[derive(Properties, PartialEq)]
pub struct TournamentManagementProperties {
    pub id: i32,
}

#[function_component]
pub fn TournamentManagement(props: &TournamentManagementProperties) -> Html {
    let id = props.id;

    let tournament = use_state(Tournament::default);

    {
        let tournament = tournament.clone();
        let client = use_foos_db_client();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    tournament.set(client.tournament(id).await.unwrap());
                });

                || {}
            },
            (),
        );
    }

    html! {
        <>
            <input type="text" placeholder="Tournament Name" value={tournament.name.clone()} /><button>{"Rename"}</button>
        </>
    }
}
