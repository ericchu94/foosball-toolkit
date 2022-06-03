#![feature(type_alias_impl_trait)]
#![feature(const_option_ext)]

mod components;
pub mod foos_db_client;
mod hooks;
mod models;

use components::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
