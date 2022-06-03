use std::collections::HashMap;
use std::time::Instant;

use log::info;
use thiserror::Error;
use time::OffsetDateTime;

use crate::database::{Database, DatabaseError};
use crate::models::*;

use super::{EloRatingCalculator, RatingCalculator};

type Result<T> = std::result::Result<T, RatingServiceError>;

#[derive(Error, Debug)]
pub enum RatingServiceError {
    #[error("sqlx error `{0}`")]
    Database(#[from] DatabaseError),
}

pub struct RatingService {
    database: Database,
    calculator: EloRatingCalculator,
}

impl RatingService {
    pub fn new(database: Database) -> Self {
        let calculator = EloRatingCalculator::new();
        Self {
            database,
            calculator,
        }
    }

    async fn delete_ratings_after_timestamp(&self, timestamp: OffsetDateTime) -> Result<()> {
        self.database
            .delete_ratings_after_timestamp(timestamp)
            .await?;

        Ok(())
    }

    async fn get_first_match_without_rating(&self) -> Result<Match> {
        Ok(self.database.get_first_match_without_rating().await?)
    }

    async fn get_timestamp_to_purge(&self) -> OffsetDateTime {
        self.get_first_match_without_rating()
            .await
            .map(|m| m.timestamp)
            .unwrap_or(OffsetDateTime::UNIX_EPOCH)
    }

    async fn delete_inconsistent_ratings(&self) -> Result<()> {
        let timestamp = self.get_timestamp_to_purge().await;

        self.delete_ratings_after_timestamp(timestamp).await
    }

    async fn get_matches_without_ratings(&self) -> Result<Vec<Match>> {
        Ok(self.database.get_matches_without_ratings().await?)
    }

    async fn get_player_ratings_for_match(
        &self,
        latest_ratings: &HashMap<i32, i32>,
        player_matches: &[PlayerMatch],
    ) -> Result<(Vec<(i32, i32)>, Vec<(i32, i32)>)> {
        let map = player_matches.iter().fold(HashMap::new(), |mut acc, pm| {
            acc.entry(pm.team).or_insert(vec![]).push(pm.player_id);
            acc
        });

        let team1_ratings = map[&Team::Team1]
            .iter()
            .map(|id| (*id, *latest_ratings.get(id).unwrap_or(&1500)))
            .collect();
        let team2_ratings = map[&Team::Team2]
            .iter()
            .map(|id| (*id, *latest_ratings.get(id).unwrap_or(&1500)))
            .collect();

        Ok((team1_ratings, team2_ratings))
    }

    async fn compute_ratings_for_match(
        &self,
        latest_ratings: &mut HashMap<i32, i32>,
        player_matches: &[PlayerMatch],
        m: Match,
        games: &[Game],
    ) -> Result<Vec<Rating>> {
        let (team1_ratings, team2_ratings) = self
            .get_player_ratings_for_match(latest_ratings, player_matches)
            .await?;

        let before1 = team1_ratings.iter().map(|pair| pair.1).sum::<i32>() as f64
            / team1_ratings.len() as f64;
        let before2 = team2_ratings.iter().map(|pair| pair.1).sum::<i32>() as f64
            / team2_ratings.len() as f64;

        let delta = self
            .calculator
            .calculate_match(before1, before2, games)
            .round() as i32;

        let ratings = team1_ratings
            .into_iter()
            .map(|p| Self::create_rating(p.0, p.1, delta, m.id))
            .chain(
                team2_ratings
                    .into_iter()
                    .map(|p| Self::create_rating(p.0, p.1, -delta, m.id)),
            )
            .collect::<Vec<Rating>>();

        for r in ratings.iter() {
            latest_ratings.insert(r.player_id, r.after_rating);
        }

        Ok(ratings)
    }

    fn create_rating(player_id: i32, before_rating: i32, delta: i32, match_id: i32) -> Rating {
        Rating {
            id: 0,
            player_id,
            match_id,
            before_rating,
            after_rating: before_rating + delta,
        }
    }

    async fn compute(&self) -> Result<()> {
        let start = Instant::now();

        let matches = self.get_matches_without_ratings().await?;

        let mut latest_ratings = self
            .database
            .get_latest_rating_for_all_players()
            .await?
            .into_iter()
            .map(|r| (r.player_id, r.after_rating))
            .collect::<HashMap<i32, i32>>();

        let match_ids = matches.iter().map(|m| m.id).collect::<Vec<i32>>();

        let player_matches = self
            .database
            .get_player_matches_by_match_ids(&match_ids)
            .await?
            .into_iter()
            .fold(HashMap::new(), |mut acc, pm| {
                acc.entry(pm.match_id).or_insert(vec![]).push(pm);
                acc
            });

        let games = self
            .database
            .get_games_by_match_ids(&match_ids)
            .await?
            .into_iter()
            .fold(HashMap::new(), |mut acc, g| {
                acc.entry(g.match_id).or_insert(vec![]).push(g);
                acc
            });

        let mut ratings = vec![];

        for m in matches {
            let player_matches = &player_matches[&m.id];
            let games = &games[&m.id];

            ratings.append(
                &mut self
                    .compute_ratings_for_match(&mut latest_ratings, player_matches, m, games)
                    .await?,
            );
        }

        let len = ratings.len();

        self.database.create_ratings(ratings).await?;

        let end = Instant::now();

        info!(
            "{} ratings computed in {} milliseconds",
            len,
            (end - start).as_millis()
        );

        Ok(())
    }

    pub async fn recompute_all(&self) -> Result<()> {
        self.database.delete_all_ratings().await?;

        self.compute().await
    }

    pub async fn compute_all(&self) -> Result<()> {
        self.delete_inconsistent_ratings().await?;

        self.compute().await
    }
}
