use std::collections::HashMap;

use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tournament {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
    pub name: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created: OffsetDateTime,
    pub groups: Vec<Group>,
    pub players: Vec<Player>,
    pub teams: Vec<Team>,
    pub rounds: Vec<Round>,
    pub ko: Vec<Ko>,
    pub mode: String,
    pub num_rounds: u32,
    pub options: TournamentOptions,
    pub name_type: u32,
    pub version: String,
    pub started: bool,
    pub last_transaction_timestamp: u64,
    pub last_transaction: u32,
    pub sport: Sport,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
    pub teams: Vec<TeamRef>,
    pub rounds: Vec<GroupRound>,
}

#[derive(Deserialize, Debug)]
pub struct GroupRound {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
    pub name: String,
    pub plays: Vec<PlayRef>,
}

#[derive(Deserialize, Debug)]
pub struct PlayRef {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
    pub meta: ParticipantMeta,
    #[serde(rename = "_name")]
    pub name: String,
    pub weight: u32,
    pub start_index: u32,
    pub removed: bool,
    pub marked_for_removal: Option<bool>,
    pub deactivated: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantMeta {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
    #[serde(rename = "_addedLater")]
    pub added_later: bool,
    pub added_in_round: u32,
    pub had_bye: bool,
    pub table_index: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
    pub meta: ParticipantMeta,
    #[serde(rename = "_name")]
    pub name: Option<String>,
    pub start_index: u32,
    #[serde(default)]
    pub players: Vec<PlayerRef>,
    pub removed: bool,
    pub marked_for_removal: Option<bool>,
    pub deactivated: bool,
}

#[derive(Deserialize, Debug)]
pub struct PlayerRef {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct Round {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
    pub name: String,
    pub plays: Vec<Play>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Play {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
    pub valid: bool,
    pub team1: Option<TeamRef>,
    pub team2: Option<TeamRef>,
    pub disciplines: Vec<Discipline>,
    pub time_start: Option<u64>,
    pub time_end: Option<u64>,
    pub deactivated: bool,
    #[serde(default)]
    pub tables: Vec<TableRef>,
    pub team1bye: bool,
    pub team2bye: bool,
    pub winner: Option<u32>,
    pub team1_result: Option<u32>,
    pub team2_result: Option<u32>,
    pub skipped: bool,
    pub round_id: Option<String>,
    pub ko_id: Option<String>,
    pub level_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TeamRef {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Discipline {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
    pub sets: Vec<Result>,
    pub team1_confirmed: bool,
    pub team2_confirmed: bool,
    pub play_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Result {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
    pub team1: Option<u32>,
    pub team2: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct TableRef {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ko {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
    pub levels: Vec<Level>,
    pub left_levels: Vec<Level>,
    pub third: Level,
    pub size: u32,
    pub third_place: bool,
    pub double: bool,
    pub team_up: bool,
    pub lord_have_mercy: bool,
    pub options: TournamentOptions,
    pub finished: bool,
}

#[derive(Deserialize, Debug)]
pub struct Level {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
    pub plays: Vec<Play>,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TournamentOptions {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
    pub name: Option<String>,
    pub num_points: u32,
    pub num_sets: u32,
    pub two_ahead: bool,
    pub fast_input: bool,
    pub points_win: u32,
    pub points_draw: u32,
    pub fair_shuffle: bool,
    pub disciplines: Vec<DisciplineOptions>,
    pub tables: Vec<Table>,
    pub tables_per_play: u32,
    pub has_disciplines: bool,
    pub max_lost_games: u32,
    pub draw: bool,
    pub bye_rating: bool,
    #[serde(default)]
    pub table_config: Vec<TableConfig>,
    pub multi_table_tournament: bool,
    pub use_close_game_rating: bool,
    pub close_game_difference: u32,
    pub close_game_points_win: u32,
    #[serde(rename = "closeGamePointsLoose")]
    pub close_game_points_lose: u32,
    pub num_players_per_team: u32,
    pub dyp_mode: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DisciplineOptions {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
    pub num_points: u32,
    pub num_sets: u32,
    pub two_ahead: bool,
    pub fast_input: bool,
}

#[derive(Deserialize, Debug)]
pub struct Table {
    #[serde(rename = "_id")]
    pub id: String,
    pub r#type: String,
    pub name: String,
    pub deactivated: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TableConfig {
    #[serde(rename = "_id")]
    pub id: String,
    pub ignore_sort: bool,
    pub visible: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sport {
    pub default_options: DefaultOptions,
    pub table_config: HashMap<String, Vec<TableConfig>>,
    pub has_goals: Option<bool>,
    pub has_fast_entry: Option<bool>,
    pub has_sets: bool,
    pub has_draw: bool,
    pub has_points: bool,
    pub has_disciplines: bool,
    pub has_close_game_rating: bool,
    pub has_bye_rating: bool,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefaultOptions {
    pub num_points: u32,
    pub num_sets: Option<u32>,
    pub points_win: u32,
    pub points_draw: u32,
    pub draw: bool,
    pub fast_input: Option<bool>,
}
