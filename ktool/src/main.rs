pub mod ktool;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use std::iter;

    use crate::ktool::{Play, Tournament};

    #[test]
    fn test() {
        let s = include_str!("05_15_2022.ktool");
        let tournament: Tournament = serde_json::from_str(s).unwrap();
        dbg!(tournament);
    }

    #[test]
    fn test_2() {
        let s = include_str!("05_16_2022.ktool");
        let tournament: Tournament = serde_json::from_str(s).unwrap();
        dbg!(&tournament);

        let get_player = |player_id: &str| {
            tournament
                .players
                .iter()
                .find(|player| player.id == player_id)
                .unwrap()
        };

        let get_players_from_team = |team_id: &str| {
            let team = tournament
                .teams
                .iter()
                .find(|team| team.id == team_id)
                .unwrap();
            team.players
                .iter()
                .map(|player| get_player(&player.id).name.clone())
                .collect::<Vec<String>>()
        };

        #[derive(Debug)]
        enum Winner {
            None,
            Team1,
            Team2,
            Draw,
        }

        let get_winner = |play: &Play| match play.winner {
            Some(idx) => {
                if idx == 1 {
                    Winner::Team1
                } else if idx == 2 {
                    Winner::Team2
                } else {
                    panic!()
                }
            }
            None => {
                let (r1, r2) = play
                    .disciplines
                    .iter()
                    .map(|discipline| {
                        discipline
                            .sets
                            .iter()
                            .map(|result| (result.team1, result.team2))
                            .fold((0, 0), |acc, item| (acc.0 + item.0, acc.1 + item.1))
                    })
                    .fold((0, 0), |acc, item| (acc.0 + item.0, acc.1 + item.1));

                if r1 > r2 {
                    Winner::Team1
                } else if r2 > r1 {
                    Winner::Team2
                } else if r1 == 0 && r2 == 0 {
                    Winner::None
                } else {
                    Winner::Draw
                }
            }
        };

        println!("monster dyp");

        let mut plays = tournament
            .rounds
            .iter()
            .flat_map(|round| round.plays.iter())
            .collect::<Vec<&Play>>();
        plays.sort_by_key(|play| play.time_end);

        for play in plays {
            if let (Some(t1), Some(t2)) = (play.team1.as_ref(), play.team2.as_ref()) {
                let players1 = get_players_from_team(&t1.id);
                let players2 = get_players_from_team(&t2.id);
                let winner = get_winner(play);

                println!(
                    "{:?} {:?} vs {:?}. Winner: {:?}",
                    play.time_end, players1, players2, winner
                );
            }
        }

        println!("elimination");

        let mut plays = tournament
            .ko
            .iter()
            .flat_map(|ko| {
                ko.levels
                    .iter()
                    .chain(ko.left_levels.iter().chain(iter::once(&ko.third)))
            })
            .flat_map(|level| level.plays.iter())
            .collect::<Vec<&Play>>();
        plays.sort_by_key(|play| play.time_end);

        for play in plays {
            if let (Some(t1), Some(t2)) = (play.team1.as_ref(), play.team2.as_ref()) {
                let players1 = get_players_from_team(&t1.id);
                let players2 = get_players_from_team(&t2.id);
                let winner = get_winner(play);

                println!(
                    "{:?} {:?} vs {:?}. Winner: {:?}",
                    play.time_end, players1, players2, winner
                );
            }
        }
    }
}
