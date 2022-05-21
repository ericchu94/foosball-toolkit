use actix_web::{
    get,
    web::{self, Data, Json, Path, ServiceConfig},
    Responder, Result,
};

use crate::database::Database;

#[get("/{id}")]
async fn get_match_data(database: Data<Database>, path: Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let match_data = database.get_match_data(id).await?;
    Ok(Json(match_data))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/match_data").service(get_match_data));
}
