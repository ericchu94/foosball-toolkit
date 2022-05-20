use actix_web::{
    get, post,
    web::{self, Data, Json, ServiceConfig, Path},
    HttpResponse, Responder, Result,
};

use crate::{database::Database, models::Tournament};

#[get("")]
async fn get_tournaments(database: Data<Database>) -> Result<impl Responder> {

    Ok(Json(database.get_tournaments().await?))
}

#[get("/{id}")]
async fn get_tournament_by_id(database: Data<Database>, path: Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    Ok(Json(database.get_tournament_by_id(id).await?))
}

#[post("")]
async fn create_tournament(
    tournament: Json<Tournament>,
    database: Data<Database>,
) -> Result<impl Responder> {
    database.create_tournament(tournament.0).await?;

    Ok(HttpResponse::Ok())
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/tournament")
            .service(get_tournaments)
            .service(get_tournament_by_id)
            .service(create_tournament),
    );
}
