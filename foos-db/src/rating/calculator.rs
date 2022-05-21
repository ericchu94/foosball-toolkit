use elo::EloRank;

use crate::models::Winner;

pub type Outcome = Winner;

pub trait RatingCalculator {
    fn calculate(&self, rating1: i32, rating2: i32, result: Outcome) -> (i32, i32);
}

pub struct EloRatingCalculator {
    elo_rank: EloRank,
}

impl EloRatingCalculator {
    pub fn new() -> Self {
        let elo_rank = EloRank { k: 40 };
        Self { elo_rank }
    }
}

impl RatingCalculator for EloRatingCalculator {
    fn calculate(&self, rating1: i32, rating2: i32, outcome: Outcome) -> (i32, i32) {
        let (a, b) = match outcome {
            Winner::Team1 => (rating1, rating2),
            Winner::Team2 => (rating2, rating1),
            _ => return (rating1, rating2),
        };

        let (a, b) = self.elo_rank.calculate(a as f64, b as f64);
        let (a, b) = (a as i32, b as i32);
        
        if outcome == Winner::Team2 {
            (b, a)
        } else {
            (a, b)
        }
    }
}
