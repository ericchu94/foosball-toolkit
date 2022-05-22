use yew::prelude::*;

use super::*;

use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
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
                Route::Home => html! { <Home /> },
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
