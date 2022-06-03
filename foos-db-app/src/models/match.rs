use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Match {
    #[serde(default)]
    pub id: i32,
    pub tournament_id: Option<i32>,
    pub timestamp: OffsetDateTime,
    pub winner: Winner,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Winner {
    None,
    Team1,
    Team2,
    Draw,
}
