mod standings;
mod status;
mod next;

use std::time::Duration;

use kickertool_data::*;
use rxrust::prelude::*;
use yew::prelude::*;
use web_sys::HtmlInputElement;

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

    let table_number = use_state(|| 1);

    let table_number_on_input = {
        let table_number = table_number.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_unchecked_into::<HtmlInputElement>();
            table_number.set(target.value().parse().unwrap());
        })
    };

    let status_font_size = use_state(|| 48);

    let status_font_size_on_input = {
        let status_font_size = status_font_size.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_unchecked_into::<HtmlInputElement>();
            status_font_size.set(target.value().parse().unwrap());
        })
    };

    let headings_font_size = use_state(|| 32);

    let headings_font_size_on_input = {
        let headings_font_size = headings_font_size.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_unchecked_into::<HtmlInputElement>();
            headings_font_size.set(target.value().parse().unwrap());
        })
    };

    let lists_font_size = use_state(|| 24);

    let lists_font_size_on_input = {
        let lists_font_size = lists_font_size.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_unchecked_into::<HtmlInputElement>();
            lists_font_size.set(target.value().parse().unwrap());
        })
    };

    let match1 = kickertool_data
        .tables
        .iter()
        .find(|table| table.number == *table_number)
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
            .kt-window * {
                font-size: 48px;
            }
            .kt-window .raw {
                font-size: 24px;
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
                padding: 25px;
                display: grid;
            }
            .kt-logo div {
                place-self: center;
                font-size: 48px;
                text-align: center;
            }
            "}
            {format!("
            .kt h1 {{
                font-size: {}px;
            }}
            ", *headings_font_size)}
            {format!("
            .kt li {{
                font-size: {}px;
            }}
            ", *lists_font_size)}</style>
            <div class="kt">
                <div class="kt-window">
                <div>
                    <label for="table-number">{"Table number: "}</label>
                    <input id="table-number" type="number" value={table_number.to_string()} oninput={table_number_on_input} />
                </div>
                <div>
                    <label for="status-font-size">{"Status Font Size (px): "}</label>
                    <input id="status-font-size" type="number" value={status_font_size.to_string()} oninput={status_font_size_on_input} />
                </div>
                <div>
                    <label for="headings-font-size">{"Headings Font Size (px): "}</label>
                    <input id="headings-font-size" type="number" value={headings_font_size.to_string()} oninput={headings_font_size_on_input} />
                </div>
                <div>
                    <label for="lists-font-size">{"Lists Font Size (px): "}</label>
                    <input id="lists-font-size" type="number" value={lists_font_size.to_string()} oninput={lists_font_size_on_input} />
                </div>
                    <div class="raw">
                    {format!("{:?}", *kickertool_data)}
                    </div>
                </div>
                <div class="kt-status">
                    <Status r#match={match1} font_size={*status_font_size} />
                </div>
                <div class="kt-standings">
                    <Standings {standings} />
                </div>
                <div class="kt-next">
                    <Next {next_matches} />
                </div>
                <div class="kt-logo">
                    <div>
                        <a href="https://github.com/ericchu94/foosball-toolkit">{"Foosball Toolkit"}</a>{" by Eric Chu"}
                    </div>
                </div>
            </div>
        </>
    }
}
