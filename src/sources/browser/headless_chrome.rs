use std::{path::PathBuf, sync::Arc, time::Duration};

use futures::executor::ThreadPool;
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use rxrust::prelude::*;

pub struct HeadlessChromeSource {
    browser: Browser,
    html_observable: HtmlObservable,
}

impl HeadlessChromeSource {
    pub fn new() -> Self {
        let browser = get_browser();
        let tab = browser.wait_for_initial_tab().unwrap();
        let html_observable = get_html_observable(tab);
        Self {
            browser,
            html_observable,
        }
    }

    pub fn first_tab(&self) -> Option<Arc<Tab>> {
        let tabs = self.browser.get_tabs().lock().ok()?;

        tabs.get(0).cloned()
    }

    pub fn html_observable(&self) -> HtmlObservable {
        self.html_observable.clone()
    }
}

pub type HtmlObservable = impl Observable<Item = String, Err = ()> + Clone + SharedObservable;

fn get_html_observable(tab: Arc<Tab>) -> HtmlObservable {
    let scheduler = ThreadPool::new().unwrap();
    observable::interval(Duration::from_secs(1), scheduler)
        .flat_map(move |_| observable::of_option(get_html(tab.clone())))
        .distinct_until_changed()
        .share()
        .into_shared()
}

fn get_html(tab: Arc<Tab>) -> Option<String> {
    let remote_object = tab
        .evaluate("document.documentElement.outerHTML", false)
        .ok()?;

    let json = remote_object.value?;
    let str = json.as_str()?;

    Some(str.to_owned())
}

fn get_browser() -> Browser {
    let launch_options = LaunchOptionsBuilder::default()
        .headless(false)
        .user_data_dir(get_user_data_dir())
        .idle_browser_timeout(Duration::from_secs(3600))
        .build()
        .unwrap();

    Browser::new(launch_options).unwrap()
}

#[cfg(target_os = "windows")]
fn get_user_data_dir() -> Option<PathBuf> {
    dirs_sys::known_folder_local_app_data().map(|mut path_buf| {
        path_buf.push("Google");
        path_buf.push("Chrome");
        path_buf.push("User Data");
        path_buf
    })
}

#[cfg(test)]
mod test {
    use super::get_user_data_dir;

    #[test]
    fn test_get_user_data_dir() {
        assert!(get_user_data_dir()
            .unwrap()
            .ends_with(r"AppData\Local\Google\Chrome\User Data"));
    }
}
