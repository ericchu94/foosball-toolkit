mod ktool;

use std::{collections::HashMap};

use actix_multipart::Multipart;
use actix_web::{
    post,
    web::{self, Data, ServiceConfig},
    HttpResponse, Responder, Result,
};
use futures::{stream, StreamExt};

use crate::{
    controllers::import::ktool::{import_kt, parse},
    database::Database,
};

#[post("")]
async fn import(payload: Multipart, database: Data<Database>) -> Result<impl Responder> {
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

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/import").service(import));
}
