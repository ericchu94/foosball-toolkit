use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Player {
    #[serde(default)]
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PlayerWithTournamentCount {
    pub player: Player,
    pub tournament_count: i32,
}
