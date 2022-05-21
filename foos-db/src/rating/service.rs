use std::collections::HashMap;

use futures::{stream, StreamExt};
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

    /// This retrieves the current player rating.
    /// Ratings need to be consistent first!
    async fn get_player_rating(&self, player_id: i32) -> i32 {
        self.database
            .get_latest_rating_of_player(player_id)
            .await
            .map(|r| r.after_rating)
            .unwrap_or(1500)
    }

    async fn get_matches_without_ratings(&self) -> Result<Vec<Match>> {
        Ok(self.database.get_matches_without_ratings().await?)
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

        let team1_ratings = stream::iter(map[&Team::Team1].iter())
            .then(|&player_id| async move { (player_id, self.get_player_rating(player_id).await) })
            .collect::<Vec<(i32, i32)>>()
            .await;
        let team2_ratings = stream::iter(map[&Team::Team2].iter())
            .then(|&player_id| async move { (player_id, self.get_player_rating(player_id).await) })
            .collect::<Vec<(i32, i32)>>()
            .await;

        Ok((team1_ratings, team2_ratings))
    }

    async fn compute_ratings_for_match(&self, m: Match) -> Result<()> {
        let (team1_ratings, team2_ratings) = self.get_player_ratings_for_match(m.id).await?;

        let before1 =
            team1_ratings.iter().map(|pair| pair.1).sum::<i32>() / team1_ratings.len() as i32;
        let before2 =
            team1_ratings.iter().map(|pair| pair.1).sum::<i32>() / team2_ratings.len() as i32;

        let (after1, after2) = self.calculator.calculate(before1, before2, m.winner);

        let delta1 = after1 - before1;
        let delta2 = after2 - before2;

        for p1 in team1_ratings {
            let rating = Self::create_rating(p1.0, p1.1, delta1, m.id);
            self.database.create_rating(rating).await?;
        }

        for p1 in team2_ratings {
            let rating = Self::create_rating(p1.0, p1.1, delta2, m.id);
            self.database.create_rating(rating).await?;
        }

        Ok(())
    }

    fn create_rating(player_id: i32, before_rating: i32, delta: i32, match_id: i32) -> Rating {
        Rating {
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
