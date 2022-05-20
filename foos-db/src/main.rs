mod controllers;
mod database;
mod models;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer, ResponseError};
use database::Database;

impl ResponseError for database::DatabaseError {}

fn cors() -> Cors {
    Cors::default().allow_any_origin()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = Database::new().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(cors())
            .app_data(Data::new(database.clone()))
            .configure(controllers::config)
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
