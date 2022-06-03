use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Tournament {
    #[serde(default)]
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub source: String,
    pub import_id: i32,
}
