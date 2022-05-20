use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Tournament {
    #[serde(default)]
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub source: String,
}
