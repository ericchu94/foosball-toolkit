mod fast;
mod ktool;

use std::time::Instant;

use actix_multipart::{Multipart, MultipartError};
use actix_web::{
    get,
    http::header::{
        self, Charset, ContentDisposition, ContentType, DispositionParam, ExtendedValue,
    },
    post,
    web::{self, Data, Path, ServiceConfig},
    HttpResponse, Responder, ResponseError, Result,
};
use futures::{future, stream, StreamExt, TryStreamExt};
use log::info;
use zip::result::ZipError;

use crate::{
    controllers::import::ktool::{import_kt, parse},
    database::{Database, DatabaseError},
    models::Import,
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
    let start = Instant::now();
    let (file_name, file) = parse_payload(payload).await?;
    let end = Instant::now();
    info!(
        "ktool buffer took {} milliseconds",
        (end - start).as_millis()
    );

    let start = Instant::now();
    let kt = parse(&file)?;
    let end = Instant::now();
    info!(
        "ktool parsing took {} milliseconds",
        (end - start).as_millis()
    );

    let import_id = database
        .create_import(Import {
            file_name,
            file,
            ..Import::default()
        })
        .await?
        .id;

    let start = Instant::now();
    import_kt(database, kt, import_id).await?;
    let end = Instant::now();
    info!(
        "ktool import took {} milliseconds",
        (end - start).as_millis()
    );

    Ok(format!("{} milliseconds", (end - start).as_millis()))
}

async fn import_fast_impl(payload: Multipart, database: Data<Database>) -> Result<impl Responder> {
    let start = Instant::now();
    let (file_name, file) = parse_payload(payload).await?;
    let end = Instant::now();
    info!(
        "fast buffer took {} milliseconds",
        (end - start).as_millis()
    );

    let start = Instant::now();
    let fast = fast::parse(&mut file.as_slice())?;
    let end = Instant::now();
    info!(
        "fast parsing took {} milliseconds",
        (end - start).as_millis()
    );

    let import_id = database
        .create_import(Import {
            file_name,
            file,
            ..Import::default()
        })
        .await?
        .id;

    let start = Instant::now();
    fast::import_fast(database, fast, import_id).await?;
    let end = Instant::now();
    info!(
        "fast import took {} milliseconds",
        (end - start).as_millis()
    );

    Ok(format!("{} milliseconds", (end - start).as_millis()))
}

async fn parse_payload(payload: Multipart) -> ImportResult<(String, Vec<u8>)> {
    Box::pin(
        payload
            .map_err(ImportError::from)
            .and_then(|field| async {
                let name = field.name().to_owned();
                let file_name = field
                    .content_disposition()
                    .get_filename()
                    .unwrap()
                    .to_owned();
                let value = field
                    .map_ok(|b| stream::iter(b).map(ImportResult::Ok))
                    .try_flatten()
                    .try_collect::<Vec<u8>>()
                    .await?;
                Ok((name, (file_name, value)))
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
    let (file_name, file) = parse_payload(payload).await?;

    let fast = fast::parse(&mut file.as_slice())?;

    let import_id = database
        .create_import(Import {
            file_name,
            file,
            ..Import::default()
        })
        .await?
        .id;

    let start = Instant::now();

    fast::import_fast_init(database, fast).await?;

    let end = Instant::now();

    info!(
        "fast-init import took {} milliseconds",
        (end - start).as_millis()
    );
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

#[get("/{id}")]
async fn get(database: Data<Database>, path: Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();

    let i = database.get_import(id).await?;

    let param = if i.file_name.is_ascii() {
        DispositionParam::Filename(i.file_name)
    } else {
        DispositionParam::FilenameExt(ExtendedValue {
            charset: Charset::Ext(String::from("UTF-8")),
            language_tag: None,
            value: i.file_name.into_bytes(),
        })
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::octet_stream())
        .insert_header(ContentDisposition {
            disposition: header::DispositionType::Attachment,
            parameters: vec![param],
        })
        .body(i.file))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/import")
            .service(import)
            .service(import_ktool)
            .service(import_fast)
            .service(import_fast_init)
            .service(get)
            .service(clear),
    );
}
