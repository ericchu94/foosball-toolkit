use actix_web::{
    get,
    web::{self, Data, Json, Query, ServiceConfig},
    Responder, Result,
};

use serde::Deserialize;

use crate::database::Database;

#[derive(Deserialize)]
struct MatchQuery {
    limit: Option<i32>,
    offset: Option<i32>,
}

#[get("")]
async fn get_matches(database: Data<Database>, query: Query<MatchQuery>) -> Result<impl Responder> {
    let limit = query.limit.unwrap_or(100);
    let offset = query.offset.unwrap_or(0);
    let matches = database.get_matches(limit, offset).await?;
    Ok(Json(matches))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/match").service(get_matches));
}
