use yew::prelude::*;

#[function_component]
pub fn Kickertool() -> Html {
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
                    {"Kickertool"}
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
