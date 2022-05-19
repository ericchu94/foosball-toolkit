mod fast;
mod ktool;

use std::collections::HashMap;

use actix_multipart::Multipart;
use actix_web::{
    post,
    web::{self, Data, ServiceConfig},
    HttpResponse, Responder, Result, ResponseError,
};
use futures::{stream, StreamExt};
use zip::result::ZipError;

use crate::{
    controllers::import::ktool::{import_kt, parse},
    database::{Database, DatabaseError},
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
}

impl ResponseError for ImportError {}

async fn import_ktool_impl(payload: Multipart, database: Data<Database>) -> Result<impl Responder> {
    let map = payload
        .map(Result::unwrap)
        .then(|field| async {
            let name = field.name().to_owned();
            let value = field
                .map(Result::unwrap)
                .flat_map(stream::iter)
                .collect::<Vec<u8>>()
                .await;
            (name, value)
        })
        .collect::<HashMap<String, Vec<u8>>>()
        .await;

    let file = &map["file"];

    let kt = parse(file)?;

    import_kt(database, kt).await?;

    Ok(HttpResponse::Ok())
}

async fn import_fast_impl(payload: Multipart, database: Data<Database>) -> Result<impl Responder> {
    let map = payload
        .map(Result::unwrap)
        .then(|field| async {
            let name = field.name().to_owned();
            let value = field
                .map(Result::unwrap)
                .flat_map(stream::iter)
                .collect::<Vec<u8>>()
                .await;
            (name, value)
        })
        .collect::<HashMap<String, Vec<u8>>>()
        .await;

    let mut file = &map["file"];

    let fast = fast::parse(&mut file.as_slice())?;

    fast::import_fast(database, fast).await?;

    Ok(HttpResponse::Ok())
}

#[post("/ktool")]
async fn import_ktool(payload: Multipart, database: Data<Database>) -> Result<impl Responder> {
    import_ktool_impl(payload, database).await
}

#[post("/fast")]
async fn import_fast(payload: Multipart, database: Data<Database>) -> Result<impl Responder> {
    import_fast_impl(payload, database).await
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
            .service(import_fast),
    );
}
