mod controllers;
mod database;
mod models;

use actix_web::{web::Data, App, HttpServer, ResponseError, middleware::Logger};
use database::Database;

impl ResponseError for database::DatabaseError {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = Database::new().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(database.clone()))
            .configure(controllers::config)
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
