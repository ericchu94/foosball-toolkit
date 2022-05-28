mod controllers;
mod database;
mod models;
mod rating;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer, ResponseError};
use database::Database;
use rating::RatingService;

impl ResponseError for database::DatabaseError {}

fn cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .send_wildcard()
        .allow_any_method()
        .allow_any_header()
}

fn get_connection_string() -> String {
    std::env::var("POSTGRES_CONNECTION_STRING")
        .unwrap_or_else(|_| "postgresql://postgres@localhost".to_owned())
}

fn get_port() -> u16 {
    if let Ok(s) = std::env::var("PORT") {
        if let Ok(p) = s.parse() {
            return p;
        }
    }
    8888
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let database = Database::new(&get_connection_string()).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(cors())
            .app_data(Data::new(database.clone()))
            .app_data(Data::new(RatingService::new(database.clone())))
            .configure(controllers::config)
    })
    .bind(("0.0.0.0", get_port()))?
    .run()
    .await
}
