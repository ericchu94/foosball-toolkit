#![feature(type_alias_impl_trait)]
#![feature(const_option_ext)]

mod components;
mod hooks;
mod models;
pub mod foos_db_client;

use components::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
