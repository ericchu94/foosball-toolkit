use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use lru::LruCache;
use rocket::{http::Method, State};
use rocket_cors::{AllowedOrigins, CorsOptions};

type SafeLruCache = Arc<Mutex<LruCache<PathBuf, Vec<u8>>>>;

#[macro_use]
extern crate rocket;

#[get("/<path..>")]
fn get(path: PathBuf, cache: &State<SafeLruCache>) -> Option<Vec<u8>> {
    cache.lock().unwrap().get(&path).cloned()
}

#[post("/<path..>", data = "<data>")]
fn post(path: PathBuf, data: &[u8], cache: &State<SafeLruCache>) {
    cache.lock().unwrap().put(path, data.to_vec());
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::All)
        .allowed_methods(vec![Method::Get].into_iter().map(From::from).collect())
        .to_cors()
        .unwrap();
    let figment = rocket::Config::figment().merge(("address", "0.0.0.0"));
    rocket::build()
        .configure(figment)
        .manage(Arc::new(Mutex::new(LruCache::<PathBuf, Vec<u8>>::new(10))))
        .attach(cors)
        .mount("/", routes![get, post])
}
