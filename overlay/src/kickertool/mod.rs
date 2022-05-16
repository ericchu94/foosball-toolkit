use kickertool_data::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

async fn get_kickertool_data() -> KickertoolData {
    reqwest::get("http://localhost:8000/data")
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

#[function_component]
pub fn Kickertool() -> Html {
    let kickertool_data = use_state(KickertoolData::default);

    {
        let kickertool_data = kickertool_data.clone();
        spawn_local(async move {
            let data = get_kickertool_data().await;
            kickertool_data.set(data);
        });
    }

    html! {
        <>
            <style>{"
            .grid {
                height: 100%;
                display: grid;
                grid-template-columns: auto 20%;
                grid-template-rows: 40% 40% 20%;
            }
            .window {
                grid-column: 1;
                grid-row: 1 / 3;
                background: #fff;
            }
            .status {
                grid-column: 1;
                grid-row: 3;
                background: #eee;
            }
            .standings {
                grid-column: 2;
                grid-row: 1;
                background: #ddd;
            }
            .next {
                grid-column: 2;
                grid-row: 2;
                background: #ccc;
            }
            .logo {
                grid-column: 2;
                grid-row: 3;
                background: #bbb;
            }
            "}</style>
            <div class="grid">
                <div class="window">
                    {format!("{:?}", kickertool_data)}
                </div>
                <div class="status">
                    {"status"}
                </div>
                <div class="standings">
                </div>
                <div class="next">
                </div>
                <div class="logo">
                </div>
            </div>
        </>
    }
}
