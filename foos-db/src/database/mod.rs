use std::{cmp::Reverse, collections::HashMap};

use sqlx::{query, query_as, query_as_unchecked, PgPool};
use thiserror::Error;
use time::OffsetDateTime;

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
        let players = query_as!(Player, "SELECT * FROM player ORDER BY id ASC")
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
    ) -> Result<Match> {
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

        Ok(r#match)
    }

    pub async fn create_games(&self, games: Vec<Game>) -> Result<()> {
        let match_ids = games.iter().map(|g| g.match_id).collect::<Vec<i32>>();
        let score1s = games.iter().map(|g| g.score1).collect::<Vec<i32>>();
        let score2s = games.iter().map(|g| g.score2).collect::<Vec<i32>>();

        query!(
            "INSERT INTO game (match_id, score1, score2) SELECT * FROM UNNEST($1::int[], $2::int[], $3::int[])",
            &match_ids,
            &score1s,
            &score2s,
        )
        .execute(&self.pool)
        .await?;

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

    pub async fn create_fast_players(&self, fast_players: Vec<FastPlayer>) -> Result<()> {
        let licenses = fast_players
            .iter()
            .map(|fp| fp.license.clone())
            .collect::<Vec<String>>();
        let first_names = fast_players
            .iter()
            .map(|fp| fp.first_name.clone())
            .collect::<Vec<String>>();
        let last_names = fast_players
            .iter()
            .map(|fp| fp.last_name.clone())
            .collect::<Vec<String>>();

        query!(
            "INSERT INTO fast_player (license, first_name, last_name) SELECT * FROM UNNEST($1::text[], $2::text[], $3::text[])",
            &licenses,
            &first_names,
            &last_names,
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

    pub async fn update_player(&self, player: Player) -> Result<()> {
        query!(
            "UPDATE player SET first_name = $1, last_name = $2 WHERE id = $3",
            player.first_name,
            player.last_name,
            player.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_ratings_after_timestamp(&self, timestamp: OffsetDateTime) -> Result<()> {
        query!("DELETE FROM rating WHERE match_id IN (SELECT match_id FROM match WHERE timestamp >= $1)", timestamp)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_all_ratings(&self) -> Result<()> {
        query!("DELETE FROM rating").execute(&self.pool).await?;

        Ok(())
    }

    pub async fn get_first_match_without_rating(&self) -> Result<Match> {
        Ok(query_as_unchecked!(Match, "SELECT * FROM match WHERE NOT EXISTS (SELECT 1 FROM rating WHERE match_id = id) ORDER BY timestamp ASC LIMIT 1")
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn get_latest_rating_of_player(&self, player_id: i32) -> Result<Rating> {
        Ok(query_as!(Rating, "SELECT rating.* FROM rating JOIN match ON match_id = match.id WHERE player_id = $1 ORDER BY timestamp DESC LIMIT 1", player_id)
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn get_latest_rating_for_players(&self, player_ids: &[i32]) -> Result<Vec<Rating>> {
        Ok(query_as_unchecked!(
            Rating,
            "SELECT r.* FROM rating r
        JOIN (
            SELECT MAX(id) id, player_id FROM rating r
            WHERE player_id = ANY($1)
            GROUP BY player_id
        ) sub ON r.id = sub.id AND r.player_id = sub.player_id",
            player_ids
        )
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn get_rating_by_player_id_and_match_id(
        &self,
        player_id: i32,
        match_id: i32,
    ) -> Result<Rating> {
        Ok(query_as!(
            Rating,
            "SELECT * FROM rating WHERE player_id = $1 AND match_id = $2",
            player_id,
            match_id
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn get_matches_without_ratings(&self) -> Result<Vec<Match>> {
        let matches = query_as_unchecked!(
            Match,
            "SELECT * FROM match WHERE NOT EXISTS (SELECT 1 FROM rating WHERE match_id = id) ORDER BY timestamp ASC",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(matches)
    }

    pub async fn create_ratings(&self, ratings: Vec<Rating>) -> Result<()> {
        let player_ids = ratings.iter().map(|r| r.player_id).collect::<Vec<i32>>();
        let match_ids = ratings.iter().map(|r| r.match_id).collect::<Vec<i32>>();
        let before_ratings = ratings
            .iter()
            .map(|r| r.before_rating)
            .collect::<Vec<i32>>();
        let after_rating = ratings.iter().map(|r| r.after_rating).collect::<Vec<i32>>();
        query!("INSERT INTO rating (player_id, match_id, before_rating, after_rating) SELECT * FROM UNNEST($1::int[], $2::int[], $3::int[], $4::int[])",
            player_ids.as_slice(),
            match_ids.as_slice(),
            before_ratings.as_slice(),
            after_rating.as_slice()).execute(&self.pool).await?;

        Ok(())
    }

    pub async fn get_match_data(&self, match_id: i32) -> Result<MatchData> {
        let match_data_rows = query_as_unchecked!(MatchDataRow, "SELECT t.name as tournament_name, m.id, m.timestamp, m.winner, p.first_name, p.last_name, r.before_rating, r.after_rating, pm.team FROM match m
        JOIN player_match pm ON pm.match_id = m.id
        JOIN rating r on r.match_id = m.id AND r.player_id = pm.player_id
        JOIN player p on p.id = pm.player_id
        JOIN tournament t on t.id = m.tournament_id
        WHERE m.id = $1", match_id)
        .fetch_all(&self.pool).await?;

        let match_data = match_data_rows.into_iter().map(MatchData::from).collect();

        Ok(match_data)
    }

    pub async fn get_match_datas(&self, limit: i32, offset: i32) -> Result<Vec<MatchData>> {
        let match_data_rows = query_as_unchecked!(MatchDataRow,
            "SELECT t.name as tournament_name, m.id, m.timestamp, m.winner, p.first_name, p.last_name, r.before_rating, r.after_rating, pm.team FROM match m
            JOIN player_match pm ON pm.match_id = m.id
            JOIN rating r on r.match_id = m.id AND r.player_id = pm.player_id
            JOIN player p on p.id = pm.player_id
            JOIN tournament t on t.id = m.tournament_id
            WHERE m.id IN (SELECT id FROM match ORDER BY timestamp DESC LIMIT $1 OFFSET $2)", limit, offset)
        .fetch_all(&self.pool).await?;

        let map = match_data_rows.into_iter().map(MatchData::from).fold(
            HashMap::new(),
            |mut acc, match_data| {
                acc.entry(match_data.id).or_insert(vec![]).push(match_data);
                acc
            },
        );

        let mut match_datas = map
            .into_values()
            .map(|v| v.into_iter().collect::<MatchData>())
            .collect::<Vec<MatchData>>();
        match_datas.sort_by_key(|match_data| Reverse(match_data.timestamp));

        Ok(match_datas)
    }

    pub async fn get_player_data(&self, player_id: i32) -> Result<PlayerData> {
        Ok(query_as!(
            PlayerData,
            "SELECT r.player_id, p.first_name, p.last_name, r.after_rating rating FROM player p
            JOIN rating r ON r.player_id = p.id
            WHERE p.id = $1
            ORDER BY r.id DESC
            LIMIT 1",
            player_id
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn get_player_datas(&self, limit: i32, offset: i32) -> Result<Vec<PlayerData>> {
        Ok(query_as!(
            PlayerData,
            "SELECT r.player_id, s.first_name, s.last_name, r.after_rating rating FROM rating r
            JOIN (
                SELECT MAX(r.id) rating_id, r.player_id, p.first_name, p.last_name FROM player p
                JOIN rating r ON r.player_id = p.id
                GROUP BY r.player_id, p.first_name, p.last_name
            ) s ON s.rating_id = r.id AND s.player_id = r.player_id
            ORDER BY rating DESC
            LIMIT $1 OFFSET $2",
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?)
    }
}
