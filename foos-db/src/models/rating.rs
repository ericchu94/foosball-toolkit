use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Rating {
    #[serde(default)]
    pub id: i32,
    pub player_id: i32,
    pub match_id: i32,
    pub before_rating: i32,
    pub after_rating: i32,
}
