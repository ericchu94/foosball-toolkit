#![feature(type_alias_impl_trait)]

mod kickertool;

use std::{io, path::PathBuf, time::Duration};

use headless_chrome::{Browser, LaunchOptionsBuilder};

use crate::kickertool::Kickertool;

fn get_browser() -> Browser {
    let launch_options = LaunchOptionsBuilder::default()
        .headless(false)
        .user_data_dir(get_user_data_dir())
        .idle_browser_timeout(Duration::from_secs(3600))
        .build()
        .unwrap();

    Browser::new(launch_options).unwrap()
}

fn get_user_data_dir() -> Option<PathBuf> {
    dirs_sys::known_folder_local_app_data().map(|mut path_buf| {
        path_buf.push("Google");
        path_buf.push("Chrome");
        path_buf.push("User Data");
        path_buf
    })
}

fn main() {
    println!("Hello, world!");

    let browser = get_browser();

    let tab = browser.wait_for_initial_tab().unwrap();
    tab.navigate_to("https://app.kickertool.de").unwrap();

    let kickertool = Kickertool::new(tab);

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    drop(kickertool);
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::get_user_data_dir;

    #[test]
    fn test_get_user_data_dir() {
        assert_eq!(
            PathBuf::from(r"C:\Users\Eric Chu\AppData\Local\Google\Chrome\User Data"),
            get_user_data_dir().unwrap()
        );
    }
}
