use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Game {
    #[serde(default)]
    pub id: i32,
    pub match_id: i32,
    pub score1: i32,
    pub score2: i32,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.score1, self.score2)
    }
}
