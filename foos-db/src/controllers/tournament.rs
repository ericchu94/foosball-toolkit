use actix_web::{
    get, post,
    web::{self, Data, Json, ServiceConfig},
    HttpResponse, Responder, Result,
};

use crate::{database::Database, models::Tournament};

#[get("")]
async fn get_tournaments(database: Data<Database>) -> Result<impl Responder> {
    Ok(Json(database.get_tournaments().await?))
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
            .service(create_tournament),
    );
}
