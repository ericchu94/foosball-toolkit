use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use lru::LruCache;
use rocket::State;

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
    let figment = rocket::Config::figment().merge(("address", "0.0.0.0"));
    rocket::build()
        .configure(figment)
        .manage(Arc::new(Mutex::new(LruCache::<PathBuf, Vec<u8>>::new(10))))
        .mount("/", routes![get, post])
}
