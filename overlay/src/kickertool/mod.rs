mod standings;
mod status;
mod next;

use std::time::Duration;

use kickertool_data::*;
use rxrust::prelude::*;
use yew::prelude::*;

use standings::Standings;
use status::Status;
use next::Next;

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
        use_effect_with_deps(
            move |_| {
                let mut subscription =
                    observable::interval(Duration::from_secs(1), LocalSpawner {})
                        .flat_map(move |_| {
                            observable::from_future(get_kickertool_data(), LocalSpawner {})
                        })
                        .distinct_until_changed()
                        .subscribe(move |data| {
                            kickertool_data.set(data);
                        });

                move || {
                    subscription.unsubscribe();
                }
            },
            (),
        );
    }

    let match1 = kickertool_data
        .tables
        .iter()
        .find(|table| table.number == 1)
        .map(|table| &table.r#match)
        .cloned()
        .unwrap_or_default();

    let standings = kickertool_data.standings.clone();

    let next_matches = kickertool_data.next_matches.clone();

    html! {
        <>
            <style>{"
            .kt {
                height: 100%;
                display: grid;
                grid-template-columns: auto 20%;
                grid-template-rows: 40% 40% 20%;
                font-family: sans-serif;
            }
            .kt-window {
                grid-column: 1;
                grid-row: 1 / 3;
                background: #fff;
            }
            .kt-status {
                grid-column: 1;
                grid-row: 3;
                background: #eee;
            }
            .kt-standings {
                grid-column: 2;
                grid-row: 1;
                background: #ddd;
            }
            .kt-next {
                grid-column: 2;
                grid-row: 2;
                background: #ccc;
            }
            .kt-logo {
                grid-column: 2;
                grid-row: 3;
                background: #bbb;
            }
            "}</style>
            <div class="kt">
                <div class="window">
                    {format!("{:?}", *kickertool_data)}
                </div>
                <div class="kt-status">
                    <Status r#match={match1} />
                </div>
                <div class="kt-standings">
                    <Standings {standings} />
                </div>
                <div class="kt-next">
                    <Next {next_matches} />
                </div>
                <div class="kt-logo">
                </div>
            </div>
        </>
    }
}
