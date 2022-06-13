use yew::prelude::*;

use super::*;

use crate::foos_db_client::FoosDbClient;
use crate::hooks::BASE_URL;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Eq)]
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
    #[at("/management")]
    Management,
    #[at("/management/:id")]
    TournamentManagement { id: i32 },
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
                Route::Management => html! { <Management /> },
                Route::TournamentManagement { id } => html! { <TournamentManagement {id} /> },
                _ => html! { <h1>{ "404" }</h1> },
            }}
        </>
    }
}

#[function_component]
pub fn App() -> Html {
    let foos_db_client = FoosDbClient::new(BASE_URL);

    html! {
        <>
            <ContextProvider<FoosDbClient> context={foos_db_client}>
                <div class="container">
                    <BrowserRouter>
                        <Switch<Route> render={switch} />
                    </BrowserRouter>
                </div>
            </ContextProvider<FoosDbClient>>
            <script src="/assets/bootstrap.bundle.js"></script>
        </>
    }
}
