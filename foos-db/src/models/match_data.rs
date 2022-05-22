use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::{Team, Winner};

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchDataRow {
    pub id: i32,
    pub tournament_name: String,
    pub timestamp: OffsetDateTime,
    pub winner: Winner,
    pub team: Team,
    pub first_name: String,
    pub last_name: String,
    pub before_rating: i32,
    pub after_rating: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchData {
    #[serde(default)]
    pub id: i32,
    pub tournament_name: String,
    pub timestamp: OffsetDateTime,
    pub winner: Winner,
    pub team1: Vec<MatchDataPlayer>,
    pub team2: Vec<MatchDataPlayer>,
}

impl From<MatchDataRow> for MatchData {
    fn from(row: MatchDataRow) -> Self {
        let id = row.id;
        let tournament_name = row.tournament_name;
        let timestamp = row.timestamp;
        let winner = row.winner;
        let first_name = row.first_name;
        let last_name = row.last_name;
        let before_rating = row.before_rating;
        let after_rating = row.after_rating;
        let team = row.team;

        let player = MatchDataPlayer {
            first_name,
            last_name,
            before_rating,
            after_rating,
        };

        let (team1, team2) = match team {
            Team::Team1 => (vec![player], vec![]),
            Team::Team2 => (vec![], vec![player]),
        };

        MatchData {
            id,
            tournament_name,
            timestamp,
            winner,
            team1,
            team2,
        }
    }
}

impl FromIterator<MatchData> for MatchData {
    fn from_iter<T: IntoIterator<Item = MatchData>>(iter: T) -> Self {
        iter.into_iter().reduce(|mut a, mut b| {
            a.team1.append(&mut b.team1);
            a.team2.append(&mut b.team2);
            a
        }).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchDataPlayer {
    pub first_name: String,
    pub last_name: String,
    pub before_rating: i32,
    pub after_rating: i32,
}
