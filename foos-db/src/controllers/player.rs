use actix_web::{
    get, post, put,
    web::{self, Data, Json, Path, Query, ServiceConfig},
    HttpResponse, Responder, Result, Either,
};
use serde::Deserialize;

use crate::{database::Database, models::Player, rating::RatingService};

#[derive(Deserialize)]
struct GetPlayersQuery {
    tournament_id: Option<i32>,
}

#[get("")]
async fn get_players(
    database: Data<Database>,
    query: Query<GetPlayersQuery>,
) -> Result<impl Responder> {
    let players = if let Some(tournament_id) = query.tournament_id {
        Either::Left(Json(database.get_players_by_tournament_id(tournament_id).await?))
    } else {
        Either::Right(Json(database.get_players().await?))
    };
    Ok(players)
}

#[get("/{id}")]
async fn get_player_by_id(database: Data<Database>, path: Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    Ok(Json(database.get_player_by_id(id).await?))
}

#[put("/{id}")]
async fn put_player(
    database: Data<Database>,
    path: Path<i32>,
    player: Json<Player>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let mut player = player.0;
    player.id = id;
    Ok(Json(database.update_player(player).await?))
}

#[post("")]
async fn create_player(player: Json<Player>, database: Data<Database>) -> Result<impl Responder> {
    database.create_player(player.0).await?;

    Ok(HttpResponse::Ok())
}

#[derive(Deserialize)]
struct PlayerMerge {
    from: i32,
    to: i32,
}

#[post("/merge")]
async fn merge(
    database: Data<Database>,
    merge: Json<PlayerMerge>,
    rating_service: Data<RatingService>,
) -> Result<impl Responder> {
    let merge = merge.0;

    if merge.from == merge.to {
        return Ok(HttpResponse::BadRequest());
    }

    database
        .merge_and_delete_player(merge.from, merge.to)
        .await?;
    rating_service.recompute_all().await?;

    Ok(HttpResponse::Ok())
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/player")
            .service(get_players)
            .service(get_player_by_id)
            .service(create_player)
            .service(put_player)
            .service(merge),
    );
}
