use actix_web::{
    get,
    web::{self, Data, Json, ServiceConfig, Query},
    Responder, Result,
};
use serde::Deserialize;

use crate::database::Database;

#[derive(Deserialize)]
struct PlayerDataQuery {
    limit: i32,
}

#[get("")]
async fn get_player_datas(
    database: Data<Database>,
    query: Query<PlayerDataQuery>,
) -> Result<impl Responder> {
    let limit = query.limit;
    let player_datas = database.get_player_datas(limit).await?;
    Ok(Json(player_datas))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/player_data").service(get_player_datas));
}
