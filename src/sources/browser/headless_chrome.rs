use std::{path::PathBuf, sync::Arc, time::Duration};

use futures::executor::ThreadPool;
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use rxrust::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UrlHtml {
    pub url: String,
    pub html: String,
}

pub struct HeadlessChromeSource {
    browser: Arc<Browser>,
    url_html_observable: UrlHtmlObservable,
}

pub type UrlHtmlObservable =
    impl Observable<Item = Arc<UrlHtml>, Err = ()> + Clone + SharedObservable;

fn get_url_html_observable(browser: Arc<Browser>) -> UrlHtmlObservable {
    let scheduler = ThreadPool::new().unwrap();

    observable::interval(Duration::from_secs(1), scheduler)
        .flat_map(move |_| {
            observable::from_iter(
                browser
                    .get_tabs()
                    .lock()
                    .into_iter()
                    .flat_map(|tabs| {
                        tabs.iter()
                            .cloned()
                            .flat_map(|tab| {
                                let url = tab.get_url();
                                let html = get_html(tab);

                                html.map(|html| Arc::new(UrlHtml { url, html }))
                            })
                            .collect::<Vec<Arc<UrlHtml>>>()
                    })
                    .collect::<Vec<Arc<UrlHtml>>>(),
            )
        })
        .share()
        .into_shared()
}

impl HeadlessChromeSource {
    pub fn new() -> Self {
        let browser = Arc::new(get_browser());
        browser.wait_for_initial_tab().unwrap();
        let url_html_observable = get_url_html_observable(browser.clone());
        Self {
            browser,
            url_html_observable,
        }
    }

    pub fn first_tab(&self) -> Option<Arc<Tab>> {
        let tabs = self.browser.get_tabs().lock().ok()?;
        tabs.get(0).cloned()
    }

    pub fn url_html_observable(&self) -> UrlHtmlObservable {
        self.url_html_observable.clone()
    }
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
