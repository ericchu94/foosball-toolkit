use actix_web::{
    get, post,
    web::{self, Data, Json, Path, Query, ServiceConfig},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;

use crate::{database::Database, rating::RatingService};

#[post("/compute_all")]
async fn compute_all(rating_service: Data<RatingService>) -> Result<impl Responder> {
    rating_service.compute_all().await?;

    Ok(HttpResponse::Ok())
}

#[post("/recompute_all")]
async fn recompute_all(rating_service: Data<RatingService>) -> Result<impl Responder> {
    rating_service.recompute_all().await?;

    Ok(HttpResponse::Ok())
}

#[derive(Deserialize)]
struct RatingQuery {
    match_id: Option<i32>,
}

#[get("/{id}")]
async fn get_rating(
    database: Data<Database>,
    path: Path<i32>,
    query: Query<RatingQuery>,
) -> Result<impl Responder> {
    let player_id = path.into_inner();
    Ok(Json(if let Some(match_id) = query.match_id {
        database
            .get_rating_by_player_id_and_match_id(player_id, match_id)
            .await?
    } else {
        database.get_latest_rating_of_player(player_id).await?
    }))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/rating")
            .service(recompute_all)
            .service(compute_all)
            .service(get_rating),
    );
}
