mod use_tournament;
mod use_player_matches;
pub mod use_matches;
mod use_rating;
mod use_match_data;
mod use_player_datas;
mod use_match_datas;
mod use_players;
mod use_foos_db_client;

pub use use_tournament::*;
pub use use_player_matches::*;
pub use use_matches::use_matches;
pub use use_rating::*;
pub use use_match_data::use_match_data;
pub use use_match_datas::*;
pub use use_player_datas::*;
pub use use_players::*;
pub use use_foos_db_client::*;

pub const BASE_URL: &str = option_env!("BASE_URL").unwrap_or("https://foos-db.herokuapp.com");
