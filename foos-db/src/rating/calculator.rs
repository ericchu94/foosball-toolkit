use std::cmp::Ordering;

use crate::models::Game;

const K_GAME: f64 = 12f64;
const K_BO_1: f64 = 16f64;
const K_BO_3: f64 = 24f64;
const K_BO_5: f64 = 32f64;

pub trait RatingCalculator {
    fn calculate(&self, rating1: f64, rating2: f64, score: f64, k: f64) -> f64;

    fn calculate_match(&self, rating1: f64, rating2: f64, games: &[Game]) -> f64 {
        println!("rating {} {}", rating1, rating2);
        let delta_games: f64 = games
            .iter()
            .map(|g| {
                let score = g.score1 as f64 / (g.score1 + g.score2) as f64;
                let d = self.calculate(rating1, rating2, score, K_GAME);
                println!("game {}-{} awarded {}", g.score1, g.score2, d);
                d
            })
            .sum();

        let (a, b) = games
            .iter()
            .map(|g| match g.score1.cmp(&g.score2) {
                Ordering::Greater => (1, 0),
                Ordering::Less => (0, 1),
                _ => (0, 0),
            })
            .fold((0, 0), |(a, b), (x, y)| (a + x, b + y));

        let max = a.max(b);

        let k = if max < 2 {
            K_BO_1
        } else if max < 3 {
            K_BO_3
        } else {
            K_BO_5
        };

        let delta_match = self.calculate(rating1, rating2, normalize(a, b), k);
        println!("match {}-{} awarded {}", a, b, delta_match);

        delta_games + delta_match
    }
}

fn normalize(a: i32, b: i32) -> f64 {
    a as f64 / (a + b) as f64
}

pub struct EloRatingCalculator {}

impl EloRatingCalculator {
    pub fn new() -> Self {
        Self {}
    }
}

impl RatingCalculator for EloRatingCalculator {
    fn calculate(&self, rating1: f64, rating2: f64, score: f64, k: f64) -> f64 {
        let expected = 1f64 / (1f64 + 10f64.powf((rating2 - rating1) as f64 / 400f64));
        k * (score - expected)
    }
}
