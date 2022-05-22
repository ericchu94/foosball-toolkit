use actix_web::web;

mod import;
mod r#match;
mod player;
mod player_match;
mod rating;
mod tournament;
mod match_data;
mod player_data;

pub fn config(cfg: &mut web::ServiceConfig) {
    player::config(cfg);
    tournament::config(cfg);
    import::config(cfg);
    r#match::config(cfg);
    player_match::config(cfg);
    rating::config(cfg);
    match_data::config(cfg);
    player_data::config(cfg);
}
