use sqlx::{query, query_as, PgPool};
use thiserror::Error;

use crate::models::*;

pub type Result<T> = std::result::Result<T, DatabaseError>;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("sqlx error `{0}`")]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Debug, Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let pool = PgPool::connect("postgresql://postgres@localhost").await?;

        Ok(Self { pool })
    }

    pub async fn get_players(&self) -> Result<Vec<Player>> {
        let players = query_as!(Player, "SELECT * FROM player")
            .fetch_all(&self.pool)
            .await?;

        Ok(players)
    }

    pub async fn create_player(&self, player: Player) -> Result<()> {
        query!(
            "INSERT INTO player (first_name, last_name) VALUES ($1, $2)",
            player.first_name,
            player.last_name
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_tournaments(&self) -> Result<Vec<Tournament>> {
        let tournaments = query_as!(Tournament, "SELECT * FROM tournament")
            .fetch_all(&self.pool)
            .await?;

        Ok(tournaments)
    }

    pub async fn create_tournament(&self, tournament: Tournament) -> Result<()> {
        query!("INSERT INTO tournament (name) VALUES ($1)", tournament.name)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
