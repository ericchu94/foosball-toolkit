use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Tournament {
    #[serde(default)]
    pub id: i32,
    pub name: String,
}
