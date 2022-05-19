use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FastPlayer {
    #[serde(default)]
    pub id: i32,
    pub license: String,
    pub first_name: String,
    pub last_name: String,
}
