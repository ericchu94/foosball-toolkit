use yew::prelude::*;

use crate::foos_db_client::FoosDbClient;

#[hook]
pub fn use_foos_db_client() -> FoosDbClient {
    use_context::<FoosDbClient>().unwrap()
}