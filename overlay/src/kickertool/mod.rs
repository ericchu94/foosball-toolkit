use std::time::Duration;

use kickertool_data::*;
use rxrust::prelude::*;
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
        use_effect_with_deps(move |_| {
            let mut subscription = observable::interval(Duration::from_secs(1), LocalSpawner {})
                .flat_map(move |_| observable::from_future(get_kickertool_data(), LocalSpawner {}))
                .distinct_until_changed()
                .subscribe(move |data| {
                    kickertool_data.set(data);
                });

            move || {
                subscription.unsubscribe();
            }
        }, ());
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
                    {format!("{:?}", *kickertool_data)}
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
