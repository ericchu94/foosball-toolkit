use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerMatch {
    pub player_id: i32,
    pub match_id: i32,
    pub team: Team,
}

#[derive(Serialize, Deserialize, sqlx::Type, PartialEq, Eq, Hash, Debug)]
#[sqlx(type_name = "team", rename_all = "lowercase")]
pub enum Team {
    Team1,
    Team2,
}
