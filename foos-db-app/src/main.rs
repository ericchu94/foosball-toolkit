#![feature(type_alias_impl_trait)]

mod header;
mod hooks;
mod import_component;
mod match_component;
mod models;
mod nav;
mod player_name_component;
mod recent_matches;
mod matches;

use yew::prelude::*;

use header::Header;
use import_component::ImportComponent;
use recent_matches::RecentMatches;
use matches::Matches;

use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/matches")]
    Matches,
    #[at("/import")]
    Import,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    html! {
        <>
            <Header />
            {match routes {
                Route::Home => html! { <RecentMatches /> },
                Route::Matches => html! { <Matches /> },
                Route::Import => html! {
                    <ImportComponent />
                },
                Route::NotFound => html! { <h1>{ "404" }</h1> },
            }}
        </>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <div class="container">
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </div>
            <script src="assets/bootstrap.bundle.js"></script>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
