#![feature(type_alias_impl_trait)]

mod components;
mod hooks;
mod models;

use components::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
