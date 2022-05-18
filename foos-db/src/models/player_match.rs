use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlayerMatch {
    pub player_id: i32,
    pub match_id: i32,
    pub team: Team,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "team", rename_all = "lowercase")]
pub enum Team {
    Team1,
    Team2,
}
