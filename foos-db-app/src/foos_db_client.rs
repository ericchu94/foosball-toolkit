use reqwest::Client;
use thiserror::Error;

use crate::models::*;

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
            .error_for_status()?
            .json()
            .await?;
        Ok(match_datas)
    }

    pub async fn tournament(&self, id: i32) -> Result<Tournament> {
        let tournament = self
            .client
            .get(format!("{}/tournament/{id}", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(tournament)
    }

    pub async fn tournaments(&self) -> Result<Vec<Tournament>> {
        let tournaments = self
            .client
            .get(format!("{}/tournament", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(tournaments)
    }

    pub async fn put_tournament(&self, tournament: &Tournament) -> Result<()> {
        self.client
            .put(format!("{}/tournament/{}", self.base_url, tournament.id))
            .json(tournament)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub fn import(&self, id: i32) -> String {
        format!("{}/import/{id}", self.base_url)
    }

    pub async fn get_players(&self) -> Result<Vec<Player>> {
        Ok(self
            .client
            .get(format!("{}/player", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn get_players_by_tournament_id(&self, tournament_id: i32) -> Result<Vec<PlayerWithTournamentCount>> {
        Ok(self
            .client
            .get(format!("{}/player?tournament_id={tournament_id}", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}

impl PartialEq for FoosDbClient {
    fn eq(&self, other: &Self) -> bool {
        self.base_url == other.base_url
    }
}
