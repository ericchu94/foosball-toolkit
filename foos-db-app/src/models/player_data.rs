use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlayerData {
    pub player_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub rating: i32,
}
