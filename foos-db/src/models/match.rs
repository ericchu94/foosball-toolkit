use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    #[serde(default)]
    pub id: i32,
    pub tournament_id: Option<i32>,
    pub timestamp: OffsetDateTime,
    pub winner: Winner,
}

#[derive(Serialize, Deserialize, sqlx::Type, Debug, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "winner", rename_all = "lowercase")]
pub enum Winner {
    None,
    Team1,
    Team2,
    Draw,
}
