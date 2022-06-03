#![feature(type_alias_impl_trait)]

mod kickertool;

mod sinks;
mod sources;

use std::io;

use crate::{kickertool::Kickertool, sources::browser::headless_chrome::HeadlessChromeSource};

fn main() {
    println!("Hello, world!");

    let headless_chrome_source = HeadlessChromeSource::new();
    let tab = headless_chrome_source.first_tab().unwrap();

    tab.navigate_to("https://app.kickertool.de").unwrap();

    let kickertool = Kickertool::new(headless_chrome_source.url_html_observable());

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    drop(kickertool);
}
