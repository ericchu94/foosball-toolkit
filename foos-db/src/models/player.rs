use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    #[serde(default)]
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}
