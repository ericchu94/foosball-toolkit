use reqwest::Client;
use thiserror::Error;

use crate::models::MatchData;

type Result<T> = std::result::Result<T, FoosDbClientError>;

#[derive(Error, Debug)]
pub enum FoosDbClientError {
    #[error("reqwest error `{0}`")]
    Reqwest(#[from] reqwest::Error),
}

#[derive(Clone)]
pub struct FoosDbClient {
    base_url: String,
    client: Client,
}

impl FoosDbClient {
    pub fn new<S: Into<String>>(base_url: S) -> Self {
        let base_url = base_url.into();
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn match_datas(&self, offset: i32, limit: i32) -> Result<Vec<MatchData>> {
        let match_datas = self
            .client
            .get(format!(
                "{}/match_data?offset={offset}&limit={limit}",
                self.base_url
            ))
            .send()
            .await?
            .json()
            .await?;
        Ok(match_datas)
    }
}

impl PartialEq for FoosDbClient {
    fn eq(&self, other: &Self) -> bool {
        self.base_url == other.base_url
    }
}
