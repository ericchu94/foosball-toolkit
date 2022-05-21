use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::Winner;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatchData {
    #[serde(default)]
    pub id: i32,
    pub tournament_name: String,
    pub timestamp: OffsetDateTime,
    pub winner: Winner,
    pub team1: Vec<MatchDataPlayer>,
    pub team2: Vec<MatchDataPlayer>,
}

impl Default for MatchData {
    fn default() -> Self {
        Self {
            id: Default::default(),
            tournament_name: Default::default(),
            timestamp: OffsetDateTime::UNIX_EPOCH,
            winner: Winner::None,
            team1: Default::default(),
            team2: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MatchDataPlayer {
    pub first_name: String,
    pub last_name: String,
    pub before_rating: i32,
    pub after_rating: i32,
}
