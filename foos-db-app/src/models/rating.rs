use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
pub struct Rating {
    pub player_id: i32,
    pub match_id: i32,
    pub before_rating: i32,
    pub after_rating: i32,
}
