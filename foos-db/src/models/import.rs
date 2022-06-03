use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Import {
    #[serde(default)]
    pub id: i32,
    pub file: Vec<u8>,
    pub file_name: String,
}
