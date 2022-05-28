mod fast;
mod ktool;

use std::time::Instant;

use actix_multipart::{Multipart, MultipartError};
use actix_web::{
    post,
    web::{self, Data, ServiceConfig},
    HttpResponse, Responder, ResponseError, Result,
};
use futures::{future, stream, StreamExt, TryStreamExt};
use zip::result::ZipError;

use crate::{
    controllers::import::ktool::{import_kt, parse},
    database::{Database, DatabaseError},
    rating::{RatingService, RatingServiceError},
};

use thiserror::Error;

pub type ImportResult<T> = std::result::Result<T, ImportError>;

#[derive(Error, Debug)]
pub enum ImportError {
    #[error("deserialization error `{0}`")]
    Deserialization(#[from] quick_xml::de::DeError),
    #[error("database error `{0}`")]
    Database(#[from] DatabaseError),
    #[error("zip error `{0}`")]
    Zip(#[from] ZipError),
    #[error("missing field `{0}`")]
    MissingField(&'static str),
    #[error("missing player with id `{0}`")]
    MissingPlayer(u32),
    #[error("multipart error `{0}`")]
    Multipart(#[from] MultipartError),
}

impl ResponseError for ImportError {}

impl ResponseError for RatingServiceError {}

async fn import_ktool_impl(payload: Multipart, database: Data<Database>) -> Result<impl Responder> {
    let file = file_payload(payload).await?;

    let kt = parse(&file)?;

    import_kt(database, kt).await?;

    Ok(HttpResponse::Ok())
}

async fn import_fast_impl(payload: Multipart, database: Data<Database>) -> Result<impl Responder> {
    let file = file_payload(payload).await?;

    let fast = fast::parse(&mut file.as_slice())?;

    let start = Instant::now();

    fast::import_fast(database, fast).await?;

    let end = Instant::now();

    Ok(format!("{} milliseconds", (end - start).as_millis()))
}

async fn file_payload(payload: Multipart) -> ImportResult<Vec<u8>> {
    Box::pin(
        payload
            .map_err(ImportError::from)
            .and_then(|field| async {
                let name = field.name().to_owned();
                let value = field
                    .map_ok(|b| stream::iter(b).map(ImportResult::Ok))
                    .try_flatten()
                    .try_collect::<Vec<u8>>()
                    .await?;
                Ok((name, value))
            })
            .try_filter(|(name, _)| future::ready(name == "file"))
            .map_ok(|(_, data)| data),
    )
    .try_next()
    .await?
    .ok_or(ImportError::MissingField("file"))
}

async fn import_fast_init_impl(
    payload: Multipart,
    database: Data<Database>,
) -> Result<impl Responder> {
    let file = file_payload(payload).await?;

    let fast = fast::parse(&mut file.as_slice())?;

    let start = Instant::now();

    fast::import_fast_init(database, fast).await?;

    let end = Instant::now();

    println!("fast-init import took {} milliseconds", (end - start).as_millis());
    Ok(format!("{} milliseconds", (end - start).as_millis()))
}

#[post("/ktool")]
async fn import_ktool(
    payload: Multipart,
    database: Data<Database>,
    rating_service: Data<RatingService>,
) -> Result<impl Responder> {
    let response = import_ktool_impl(payload, database).await?;

    rating_service.compute_all().await?;

    Ok(response)
}

#[post("/fast")]
async fn import_fast(
    payload: Multipart,
    database: Data<Database>,
    rating_service: Data<RatingService>,
) -> Result<impl Responder> {
    let response = import_fast_impl(payload, database).await?;

    rating_service.compute_all().await?;

    Ok(response)
}

#[post("/fast-init")]
async fn import_fast_init(payload: Multipart, database: Data<Database>) -> Result<impl Responder> {
    import_fast_init_impl(payload, database).await
}

#[post("/clear")]
async fn clear(database: Data<Database>) -> Result<impl Responder> {
    database.clear_imports().await?;

    Ok(HttpResponse::Ok())
}

#[post("")]
async fn import(payload: Multipart, database: Data<Database>) -> Result<impl Responder> {
    import_ktool_impl(payload, database).await
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/import")
            .service(import)
            .service(import_ktool)
            .service(import_fast)
            .service(import_fast_init)
            .service(clear),
    );
}
