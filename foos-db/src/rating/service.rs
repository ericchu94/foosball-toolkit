use std::collections::HashMap;

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

    /// This retrieves the current player ratings.
    /// Ratings need to be consistent first!
    async fn get_latest_rating_for_players(&self, player_ids: &[i32]) -> Result<Vec<(i32, i32)>> {
        let ratings = self
            .database
            .get_latest_rating_for_players(player_ids)
            .await?
            .into_iter()
            .map(|r| (r.player_id, r.after_rating))
            .collect::<HashMap<i32, i32>>();

        Ok(player_ids
            .iter()
            .map(|id| (*id, *ratings.get(id).unwrap_or(&1500)))
            .collect())
    }

    async fn get_player_ratings_for_match(
        &self,
        match_id: i32,
    ) -> Result<(Vec<(i32, i32)>, Vec<(i32, i32)>)> {
        let player_matches = self
            .database
            .get_player_matches_by_match_id(match_id)
            .await?;

        let map = player_matches
            .into_iter()
            .fold(HashMap::new(), |mut acc, pm| {
                acc.entry(pm.team).or_insert(vec![]).push(pm.player_id);
                acc
            });

        let team1_ratings = self
            .get_latest_rating_for_players(&map[&Team::Team1])
            .await?;
        let team2_ratings = self
            .get_latest_rating_for_players(&map[&Team::Team2])
            .await?;

        Ok((team1_ratings, team2_ratings))
    }

    async fn compute_ratings_for_match(&self, m: Match) -> Result<()> {
        let (team1_ratings, team2_ratings) = self.get_player_ratings_for_match(m.id).await?;

        let before1 =
            team1_ratings.iter().map(|pair| pair.1).sum::<i32>() / team1_ratings.len() as i32;
        let before2 =
            team2_ratings.iter().map(|pair| pair.1).sum::<i32>() / team2_ratings.len() as i32;

        let (after1, after2) = self.calculator.calculate(before1, before2, m.winner);

        let delta1 = after1 - before1;
        let delta2 = after2 - before2;

        let ratings = team1_ratings
            .into_iter()
            .map(|p| Self::create_rating(p.0, p.1, delta1, m.id))
            .chain(
                team2_ratings
                    .into_iter()
                    .map(|p| Self::create_rating(p.0, p.1, delta2, m.id)),
            )
            .collect::<Vec<Rating>>();

        self.database.create_ratings(ratings).await?;

        Ok(())
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
        let matches = self.get_matches_without_ratings().await?;

        for m in matches {
            self.compute_ratings_for_match(m).await?;
        }

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
