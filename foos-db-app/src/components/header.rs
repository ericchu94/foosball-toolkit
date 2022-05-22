use yew::prelude::*;

use super::Nav;

#[function_component]
pub fn Header() -> Html {
    html! {
        <>
            <header class="d-flex justify-content-center flex-wrap">
                <span class="fs-1 me-lg-auto d-flex align-items-center">
                    <i class="bi-activity m-3 fs-1"></i>
                    {"foos-db-app"}
                </span>
                <Nav />
            </header>
        </>
    }
}
