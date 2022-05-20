#![feature(type_alias_impl_trait)]

mod recent_matches;
mod header;
mod nav;
mod models;
mod match_component;
mod hooks;
mod player_name_component;

use yew::prelude::*;

use recent_matches::RecentMatches;
use header::Header;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <div class="container">
                <Header />
                <RecentMatches />
            </div>
            <script src="assets/bootstrap.bundle.js"></script>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
