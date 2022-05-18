use actix_web::web;

mod player;
mod tournament;

pub fn config(cfg: &mut web::ServiceConfig) {
    player::config(cfg);
    tournament::config(cfg);
}