use actix_web::web;

mod player;
mod tournament;
mod import;
mod r#match;
mod player_match;

pub fn config(cfg: &mut web::ServiceConfig) {
    player::config(cfg);
    tournament::config(cfg);
    import::config(cfg);
    r#match::config(cfg);
    player_match::config(cfg);
}
