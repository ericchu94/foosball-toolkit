use actix_web::{
    get, post,
    web::{self, Data, Json, ServiceConfig, Path},
    HttpResponse, Responder, Result,
};

use crate::{database::Database, models::Player};

#[get("")]
async fn get_players(database: Data<Database>) -> Result<impl Responder> {
    Ok(Json(database.get_players().await?))
}

#[get("/{id}")]
async fn get_player_by_id(database: Data<Database>, path: Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    Ok(Json(database.get_player_by_id(id).await?))
}

#[post("")]
async fn create_player(player: Json<Player>, database: Data<Database>) -> Result<impl Responder> {
    database.create_player(player.0).await?;

    Ok(HttpResponse::Ok())
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/player")
            .service(get_players)
            .service(get_player_by_id)
            .service(create_player),
    );
}
