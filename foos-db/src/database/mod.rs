use sqlx::{query, query_as, query_as_unchecked, PgPool};
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
    pub async fn new(uri: &str) -> Result<Self> {
        let pool = PgPool::connect(uri).await?;

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

    pub async fn get_or_create_player(&self, player: Player) -> Result<Player> {
        query!(
            "INSERT INTO player (first_name, last_name) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            player.first_name,
            player.last_name
        )
        .execute(&self.pool)
        .await?;
        Ok(query_as!(
            Player,
            "SELECT * FROM player WHERE first_name = $1 AND last_name = $2",
            player.first_name,
            player.last_name
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn get_tournaments(&self) -> Result<Vec<Tournament>> {
        let tournaments = query_as!(Tournament, "SELECT * FROM tournament")
            .fetch_all(&self.pool)
            .await?;

        Ok(tournaments)
    }

    pub async fn get_tournament_by_id(&self, id: i32) -> Result<Tournament> {
        let tournament = query_as!(Tournament, "SELECT * FROM tournament WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(tournament)
    }

    pub async fn create_tournament(&self, tournament: Tournament) -> Result<()> {
        query!(
            "INSERT INTO tournament (name, source) VALUES ($1, $2)",
            tournament.name,
            tournament.source
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_or_create_tournament(&self, tournament: Tournament) -> Result<Tournament> {
        query!(
            "INSERT INTO tournament (name, source) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            tournament.name,
            tournament.source
        )
        .execute(&self.pool)
        .await?;
        Ok(query_as!(
            Tournament,
            "SELECT * FROM tournament WHERE name = $1 AND source = $2",
            tournament.name,
            tournament.source
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn create_match(&self, r#match: Match) -> Result<Match> {
        Ok(query_as_unchecked!(
            Match,
            "INSERT INTO match (tournament_id, timestamp, winner) VALUES ($1, $2, $3) RETURNING *",
            r#match.tournament_id,
            r#match.timestamp,
            r#match.winner
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn create_match_and_players(
        &self,
        r#match: Match,
        team1: Vec<Player>,
        team2: Vec<Player>,
    ) -> Result<()> {
        let r#match = self.create_match(r#match).await?;

        for player in team1 {
            let player = self.get_or_create_player(player).await?;
            let player_match = PlayerMatch {
                player_id: player.id,
                match_id: r#match.id,
                team: Team::Team1,
            };
            self.create_player_match(player_match).await?;
        }

        for player in team2 {
            let player = self.get_or_create_player(player).await?;
            let player_match = PlayerMatch {
                player_id: player.id,
                match_id: r#match.id,
                team: Team::Team2,
            };
            self.create_player_match(player_match).await?;
        }

        Ok(())
    }

    pub async fn create_player_match(&self, player_match: PlayerMatch) -> Result<()> {
        query!(
            "INSERT INTO player_match (player_id, match_id, team) VALUES ($1, $2, $3)",
            player_match.player_id,
            player_match.match_id,
            player_match.team as Team
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_fast_player(&self, fast_player: FastPlayer) -> Result<()> {
        query!(
            "INSERT INTO fast_player (license, first_name, last_name) VALUES ($1, $2, $3)",
            fast_player.license,
            fast_player.first_name,
            fast_player.last_name,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_fast_player_by_license(&self, license: &str) -> Result<FastPlayer> {
        Ok(query_as!(
            FastPlayer,
            "SELECT * FROM fast_player WHERE license = $1",
            license,
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn get_player_matches(&self) -> Result<Vec<PlayerMatch>> {
        let player_matches = query_as_unchecked!(PlayerMatch, "SELECT * FROM player_match")
            .fetch_all(&self.pool)
            .await?;

        Ok(player_matches)
    }

    pub async fn get_player_matches_by_match_id(&self, match_id: i32) -> Result<Vec<PlayerMatch>> {
        let player_matches = query_as_unchecked!(
            PlayerMatch,
            "SELECT * FROM player_match WHERE match_id = $1",
            match_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(player_matches)
    }

    pub async fn get_matches(&self, limit: i32, offset: i32) -> Result<Vec<Match>> {
        let matches = query_as_unchecked!(
            Match,
            "SELECT * FROM match ORDER BY timestamp DESC LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(matches)
    }

    pub async fn get_player_by_id(&self, id: i32) -> Result<Player> {
        let player = query_as!(Player, "SELECT * FROM player WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(player)
    }
}
