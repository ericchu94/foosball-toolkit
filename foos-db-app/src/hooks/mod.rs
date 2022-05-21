mod use_tournament;
mod use_player_matches;
mod use_player;
pub mod use_matches;
mod use_rating;

pub use use_tournament::*;
pub use use_player_matches::*;
pub use use_player::*;
pub use use_matches::use_matches;
pub use use_rating::*;

pub const BASE_URL: &str = "https://foos-db.herokuapp.com";
