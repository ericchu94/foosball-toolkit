use actix_web::{
    get,
    web::{self, Data, Json, Path, Query, ServiceConfig},
    Responder, Result,
};
use serde::Deserialize;

use crate::database::Database;

#[get("/{id}")]
async fn get_match_data(database: Data<Database>, path: Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let match_data = database.get_match_data(id).await?;
    Ok(Json(match_data))
}

#[derive(Deserialize)]
struct MatchDataQuery {
    limit: i32,
    offset: i32,
}

#[get("")]
async fn get_match_datas(
    database: Data<Database>,
    query: Query<MatchDataQuery>,
) -> Result<impl Responder> {
    let limit = query.limit;
    let offset = query.offset;
    let match_datas = database.get_match_datas(limit, offset).await?;
    Ok(Json(match_datas))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/match_data")
            .service(get_match_data)
            .service(get_match_datas),
    );
}
