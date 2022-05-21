use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::{Team, Winner};

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchDataRow {
    pub id: i32,
    pub tournament_name: String,
    pub timestamp: OffsetDateTime,
    pub winner: Winner,
    pub team: Team,
    pub first_name: String,
    pub last_name: String,
    pub before_rating: i32,
    pub after_rating: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchData {
    #[serde(default)]
    pub id: i32,
    pub tournament_name: String,
    pub timestamp: OffsetDateTime,
    pub winner: Winner,
    pub team1: Vec<MatchDataPlayer>,
    pub team2: Vec<MatchDataPlayer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchDataPlayer {
    pub first_name: String,
    pub last_name: String,
    pub before_rating: i32,
    pub after_rating: i32,
}
