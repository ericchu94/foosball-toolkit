use reqwest::{blocking::Client, IntoUrl};
use serde::Serialize;

use super::Sink;

pub struct HttpPostSink<U: IntoUrl + Clone> {
    client: Client,
    url: U,
}

impl<U: IntoUrl + Clone> HttpPostSink<U> {
    pub fn new(url: U) -> Self {
        let client = Client::new();
        Self { client, url }
    }
}

impl<T: Serialize, U: IntoUrl + Clone> Sink<T> for HttpPostSink<U> {
    fn sink(&self, data: T) {
        if let Err(err) = self.client.post(self.url.clone()).json(&data).send() {
            println!("HttpPostSink error: {err}");
        }
    }
}
