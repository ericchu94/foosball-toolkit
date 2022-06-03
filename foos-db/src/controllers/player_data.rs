use actix_web::{
    get,
    web::{self, Data, Json, Query, ServiceConfig},
    Responder, Result,
};
use serde::Deserialize;

use crate::database::Database;

#[derive(Deserialize)]
struct PlayerDataQuery {
    limit: i32,
    offset: i32,
}

#[get("")]
async fn get_player_datas(
    database: Data<Database>,
    query: Query<PlayerDataQuery>,
) -> Result<impl Responder> {
    let limit = query.limit;
    let offset = query.offset;
    let player_datas = database.get_player_datas(limit, offset).await?;
    Ok(Json(player_datas))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/player_data").service(get_player_datas));
}
