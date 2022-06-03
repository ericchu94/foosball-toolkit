use actix_web::{
    get,
    web::{self, Data, Json, Query, ServiceConfig},
    Responder, Result,
};
use serde::Deserialize;

use crate::database::Database;

#[derive(Deserialize)]
struct PlayerMatchQuery {
    match_id: i32,
}

#[get("")]
async fn get_player_matches_by_match_id(
    database: Data<Database>,
    query: Query<PlayerMatchQuery>,
) -> Result<impl Responder> {
    Ok(Json(
        database
            .get_player_matches_by_match_id(query.match_id)
            .await?,
    ))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/player_match").service(get_player_matches_by_match_id));
}
