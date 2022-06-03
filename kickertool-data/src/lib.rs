use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize, Default)]
pub struct KickertoolData {
    pub tournament_name: String,
    pub standings: Vec<String>,
    pub tables: Vec<Table>,
    pub next_matches: Vec<Match>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize, Default)]
pub struct Table {
    pub number: u8,
    pub r#match: Match,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize, Default)]
pub struct Match {
    pub team1: String,
    pub team2: String,
}
