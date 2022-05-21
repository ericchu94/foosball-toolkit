mod use_tournament;
mod use_player_matches;
mod use_player;
pub mod use_matches;
mod use_rating;
mod use_match_data;

pub use use_tournament::*;
pub use use_player_matches::*;
pub use use_player::*;
pub use use_matches::use_matches;
pub use use_rating::*;
pub use use_match_data::use_match_data;

pub const BASE_URL: &str = "https://foos-db.herokuapp.com";
