use yew::prelude::*;

use super::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <>
            <RecentMatches />
            <TopPlayers />
        </>
    }
}
