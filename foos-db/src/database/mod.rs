use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use sqlx::{query, query_as, query_as_unchecked, query_unchecked, PgPool};
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
        let players = query_as!(
            Player,
            "SELECT * FROM player ORDER BY first_name, last_name, id"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(players)
    }

    pub async fn get_players_by_tournament_id(
        &self,
        tournament_id: i32,
    ) -> Result<Vec<PlayerWithTournamentCount>> {
        let players = query!(
            "
            select distinct p.*, (SELECT COUNT(DISTINCT  m.tournament_id) tournament_count FROM player_match pm
            JOIN match m ON m.id  = pm.match_id
            WHERE pm.player_id = p.id) tournament_count from match m
            join player_match pm on m.id = pm.match_id
            join player p on p.id = pm.player_id
            where m.tournament_id = $1
            order by first_name, last_name, id;
            ",
            tournament_id
        )
        .map(|record| {
            let id = record.id;
            let first_name = record.first_name;
            let last_name = record.last_name;
            let player = Player {
                id, first_name, last_name,
            };
            let tournament_count = record.tournament_count.expect("unable to fetch player tournament count") as i32;
            PlayerWithTournamentCount {
                player, tournament_count
            }
        })
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

    pub async fn create_players(&self, players: &[Player]) -> Result<()> {
        let first_names = players
            .iter()
            .map(|p| p.first_name.clone())
            .collect::<Vec<String>>();
        let last_names = players
            .iter()
            .map(|p| p.last_name.clone())
            .collect::<Vec<String>>();
        query!(
            "INSERT INTO player (first_name, last_name) SELECT * FROM UNNEST($1::text[], $2::text[]) ON CONFLICT DO NOTHING",
            &first_names,
            &last_names        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_players_by_names(
        &self,
        names: HashSet<(&str, &str)>,
    ) -> Result<HashMap<(String, String), Player>> {
        let first_names = names
            .iter()
            .map(|n| n.0.to_owned())
            .collect::<Vec<String>>();
        let last_names = names
            .iter()
            .map(|n| n.1.to_owned())
            .collect::<Vec<String>>();
        let players = query_as!(
            Player,
            "SELECT * FROM player WHERE first_name = ANY($1) AND last_name = ANY($2)",
            &first_names,
            &last_names
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(players
            .into_iter()
            .filter(|p| names.contains(&(&p.first_name, &p.last_name)))
            .map(|p| ((p.first_name.clone(), p.last_name.clone()), p))
            .collect())
    }

    pub async fn merge_and_delete_player(&self, from: i32, to: i32) -> Result<()> {
        self.merge_player(from, to).await?;

        self.delete_player(from).await?;

        Ok(())
    }

    async fn merge_player(&self, from: i32, to: i32) -> Result<()> {
        query!(
            "UPDATE player_match SET player_id = $1 WHERE player_id = $2",
            to,
            from
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_player(&self, id: i32) -> Result<()> {
        query!("DELETE FROM player WHERE id = $1", id,)
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

    pub async fn get_tournament_by_id(&self, id: i32) -> Result<Tournament> {
        let tournament = query_as!(Tournament, "SELECT * FROM tournament WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(tournament)
    }

    pub async fn create_tournament(&self, tournament: Tournament) -> Result<()> {
        query!(
            "INSERT INTO tournament (name, source, import_id) VALUES ($1, $2, $3)",
            tournament.name,
            tournament.source,
            tournament.import_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_or_create_tournament(&self, tournament: Tournament) -> Result<Tournament> {
        query!(
            "INSERT INTO tournament (name, source, import_id) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            tournament.name,
            tournament.source,
            tournament.import_id,
        )
        .execute(&self.pool)
        .await?;
        Ok(query_as!(
            Tournament,
            "SELECT * FROM tournament WHERE name = $1 AND source = $2 AND import_id = $3",
            tournament.name,
            tournament.source,
            tournament.import_id,
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn create_matches(&self, matches: &[Match]) -> Result<Vec<Match>> {
        let tournament_ids = matches
            .iter()
            .map(|m| m.tournament_id.unwrap())
            .collect::<Vec<i32>>();
        let timestamps = matches
            .iter()
            .map(|m| m.timestamp)
            .collect::<Vec<OffsetDateTime>>();
        let winners = matches
            .iter()
            .map(|m| format!("{:?}", m.winner).to_lowercase())
            .collect::<Vec<String>>();
        Ok(query_as_unchecked!(
            Match,
            "INSERT INTO match (tournament_id, timestamp, winner) SELECT * FROM UNNEST($1::int[], $2::timestamptz[], $3::winner[]) RETURNING *",
            tournament_ids,
            timestamps,
            winners,
        )
        .fetch_all(&self.pool)
        .await?)
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

    pub async fn get_games_by_match_ids(&self, match_ids: &[i32]) -> Result<Vec<Game>> {
        let games = query_as!(
            Game,
            "SELECT * FROM game WHERE match_id = ANY($1) ORDER BY id",
            match_ids
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(games)
    }

    pub async fn create_player_matches(&self, player_matches: &[PlayerMatch]) -> Result<()> {
        let player_ids = player_matches
            .iter()
            .map(|pm| pm.player_id)
            .collect::<Vec<i32>>();
        let match_ids = player_matches
            .iter()
            .map(|pm| pm.match_id)
            .collect::<Vec<i32>>();
        let teams = player_matches
            .iter()
            .map(|pm| format!("{:?}", pm.team).to_lowercase())
            .collect::<Vec<String>>();
        query_unchecked!(
            "INSERT INTO player_match (player_id, match_id, team) SELECT * FROM UNNEST($1::int[], $2::int[], $3::team[])",
            player_ids, match_ids, teams
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

    pub async fn get_fast_players_by_licenses(
        &self,
        licenses: &[String],
    ) -> Result<Vec<FastPlayer>> {
        Ok(query_as!(
            FastPlayer,
            "SELECT * FROM fast_player WHERE license = ANY($1)",
            licenses,
        )
        .fetch_all(&self.pool)
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

    pub async fn get_player_matches_by_match_ids(
        &self,
        match_ids: &[i32],
    ) -> Result<Vec<PlayerMatch>> {
        let player_matches = query_as_unchecked!(
            PlayerMatch,
            "SELECT * FROM player_match WHERE match_id = ANY($1)",
            match_ids
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

    pub async fn update_tournament(&self, tournament: Tournament) -> Result<()> {
        query!(
            "UPDATE tournament SET name = $1, source = $2 WHERE id = $3",
            tournament.name,
            tournament.source,
            tournament.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
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

    pub async fn get_latest_rating_for_all_players(&self) -> Result<Vec<Rating>> {
        Ok(query_as_unchecked!(
            Rating,
            "SELECT r.* FROM rating r
        JOIN (
            SELECT MAX(id) id, player_id FROM rating r
            GROUP BY player_id
        ) sub ON r.id = sub.id AND r.player_id = sub.player_id",
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
            ORDER BY rating DESC, player_id
            LIMIT $1 OFFSET $2",
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn clear_imports(&self) -> Result<()> {
        query!("TRUNCATE match, player_match, player, game, tournament, rating")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_import(&self, import: Import) -> Result<Import> {
        Ok(query_as!(
            Import,
            "INSERT INTO import (file, file_name) VALUES ($1, $2) RETURNING *",
            import.file,
            import.file_name,
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn get_import(&self, id: i32) -> Result<Import> {
        Ok(query_as!(Import, "SELECT * FROM import WHERE id = $1", id,)
            .fetch_one(&self.pool)
            .await?)
    }
}
