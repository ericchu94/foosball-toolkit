mod database;
pub mod models;

use actix_web::{
    get, post,
    web::{Data, Json},
    App, HttpResponse, HttpServer, Responder, ResponseError, Result,
};
use database::Database;

use crate::models::Player;

impl ResponseError for database::DatabaseError {}

#[get("/player")]
async fn get_players(database: Data<Database>) -> Result<impl Responder> {
    Ok(Json(database.get_players().await?))
}

#[post("/player")]
async fn create_player(player: Json<Player>, database: Data<Database>) -> Result<impl Responder> {
    database.create_player(player.0).await?;

    Ok(HttpResponse::Ok())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = Database::new().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(database.clone()))
            .service(get_players)
            .service(create_player)
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
