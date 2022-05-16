mod kickertool;

use yew::prelude::*;

use kickertool::Kickertool;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <style>{"
            html,
            body {
                height: 100%;
                margin: 0;
            }
            "}</style>
            <Kickertool />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
