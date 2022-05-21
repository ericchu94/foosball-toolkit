use yew::prelude::*;

use crate::Route;

use yew_router::prelude::*;

#[function_component]
pub fn Nav() -> Html {
    html! {
        <>
            <ul class="nav align-self-center">
                <li class="nav-item">
                    <Link<Route> to={Route::Home} classes={classes!("nav-link")}>{"Home"}</Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Home} classes={classes!("nav-link", "disabled")} disabled={true}>{"Tournaments"}</Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Matches} classes={classes!("nav-link")}>{"Matches"}</Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Home} classes={classes!("nav-link", "disabled")} disabled={true}>{"Players"}</Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Import} classes={classes!("nav-link")}>{"Import"}</Link<Route>>
                </li>
            </ul>
        </>
    }
}
