use yew::prelude::*;

#[function_component]
pub fn Nav() -> Html {
    html! {
        <>
            <ul class="nav align-self-center">
                <li class="nav-item">
                    <a class="nav-link active" aria-current="page" href="#">{"Home"}</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link disabled">{"Tournaments"}</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link disabled">{"Matches"}</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link disabled">{"Players"}</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" href="#">{"Import"}</a>
                </li>
            </ul>
        </>
    }
}
