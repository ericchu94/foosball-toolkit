mod use_tournament;
mod use_player_matches;
pub mod use_matches;
mod use_rating;
mod use_match_data;
mod use_player_datas;
mod use_match_datas;

pub use use_tournament::*;
pub use use_player_matches::*;
pub use use_matches::use_matches;
pub use use_rating::*;
pub use use_match_data::use_match_data;
pub use use_match_datas::*;
pub use use_player_datas::*;

pub const BASE_URL: &str = "https://foos-db.herokuapp.com";
