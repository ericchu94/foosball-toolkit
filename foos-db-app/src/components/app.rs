use yew::prelude::*;

use super::*;

use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/matches")]
    Matches,
    #[at("/players")]
    Players,
    #[at("/import")]
    Import,
    #[at("/player_management")]
    PlayerManagement,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    html! {
        <>
            <Header />
            {match routes {
                Route::Home => html! { <Home /> },
                Route::Matches => html! { <Matches /> },
                Route::Players => html! { <Players /> },
                Route::Import => html! {
                    <ImportComponent />
                },
                Route::PlayerManagement => html! { <PlayerManagement /> },
                Route::NotFound => html! { <h1>{ "404" }</h1> },
            }}
        </>
    }
}

#[function_component]
pub fn App() -> Html {
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
