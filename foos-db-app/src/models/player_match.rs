use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerMatch {
    pub player_id: i32,
    pub match_id: i32,
    pub team: Team,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Team {
    Team1,
    Team2,
}
