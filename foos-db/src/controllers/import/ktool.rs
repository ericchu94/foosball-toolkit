use std::{collections::HashSet, iter};

use actix_web::web::Data;
use actix_web::Result;
use log::info;
use time::OffsetDateTime;

use crate::{database::Database, models::*};

pub async fn import_kt(
    database: Data<Database>,
    kt: ktool::Tournament,
    import_id: i32,
) -> Result<()> {
    let tournament = Tournament {
        name: kt.name,
        source: String::from("kickertool"),
        ..Default::default()
    };
    let tournament = database.get_or_create_tournament(tournament).await?;

    let mut plays = kt
        .rounds
        .iter()
        .flat_map(|round| round.plays.iter())
        .chain(kt.ko.iter().flat_map(|ko| {
            ko.levels
                .iter()
                .chain(ko.left_levels.iter())
                .chain(iter::once(&ko.third))
                .flat_map(|level| level.plays.iter())
        }))
        .filter(|play| play.time_end.is_some())
        .collect::<Vec<&ktool::Play>>();
    plays.sort_by_key(|play| play.time_end);

    let get_player = |player_id: &str| {
        kt.players
            .iter()
            .find(|player| player.id == player_id)
            .unwrap()
    };

    let get_players_from_team = |team_id: &str| {
        let team = kt.teams.iter().find(|team| team.id == team_id).unwrap();
        let players = team
            .players
            .iter()
            .map(|player| get_player(&player.id).name.clone())
            .collect::<Vec<String>>();

        if !players.is_empty() {
            return players;
        }

        // Try to get players from team name
        team.name
            .as_deref()
            .unwrap_or_default()
            .split('/')
            .map(|p| p.trim().to_owned())
            .collect::<Vec<String>>()
    };

    let get_games = |play: &ktool::Play| {
        play.disciplines
            .iter()
            .flat_map(|d| d.sets.iter())
            .map(|r| {
                let score1 = r.team1 as i32;
                let score2 = r.team2 as i32;

                Game {
                    score1,
                    score2,
                    ..Default::default()
                }
            })
            .collect::<Vec<Game>>()
    };

    let players = plays
        .iter()
        .flat_map(|play| {
            let t1 = play.team1.as_ref().unwrap();
            let t2 = play.team2.as_ref().unwrap();
            let players1 = get_players_from_team(&t1.id);
            let players2 = get_players_from_team(&t2.id);

            players1.into_iter().chain(players2.into_iter())
        })
        .map(|first_name| Player {
            first_name,
            ..Player::default()
        })
        .collect::<Vec<Player>>();

    let names = players
        .iter()
        .map(|p| (p.first_name.as_str(), p.last_name.as_str()))
        .collect::<HashSet<(&str, &str)>>();

    database.create_players(&players).await?;
    let players = database.get_players_by_names(names).await?;

    let (matches, mut games, mut player_matches) = plays
        .into_iter()
        .map(|play| {
            let t1 = play.team1.as_ref().unwrap();
            let t2 = play.team2.as_ref().unwrap();
            let players1 = get_players_from_team(&t1.id);
            let players2 = get_players_from_team(&t2.id);
            let winner = get_winner(play);

            let r#match = Match {
                id: 0,
                tournament_id: Some(tournament.id),
                timestamp: OffsetDateTime::from_unix_timestamp(
                    play.time_end.unwrap() as i64 / 1000,
                )
                .unwrap(),
                winner,
            };

            let games = get_games(play);

            info!(
                "{:?} {:?} vs {:?}. Winner: {:?}, Games: {:?}",
                play.time_end,
                players1,
                players2,
                winner,
                games.iter().map(|g| g.to_string()).collect::<Vec<String>>()
            );

            let team1 = players1
                .into_iter()
                .map(|p| players[&(p, String::new())].id)
                .collect::<Vec<i32>>();

            let team2 = players2
                .into_iter()
                .map(|p| players[&(p, String::new())].id)
                .collect::<Vec<i32>>();

            let player_matches = team1
                .into_iter()
                .map(|id| (id, Team::Team1))
                .chain(team2.into_iter().map(|id| (id, Team::Team2)))
                .map(|(player_id, team)| PlayerMatch {
                    player_id,
                    team,
                    match_id: 0,
                })
                .collect::<Vec<PlayerMatch>>();

            (r#match, games, player_matches)
        })
        .fold(
            (vec![], vec![], vec![]),
            |(mut matches, mut games, mut player_matches), (m, g, pm)| {
                matches.push(m);
                games.push(g);
                player_matches.push(pm);
                (matches, games, player_matches)
            },
        );

    let matches = database.create_matches(&matches).await?;

    for i in 0..matches.len() {
        let match_id = matches[i].id;
        games[i].iter_mut().for_each(|g| g.match_id = match_id);
        player_matches[i]
            .iter_mut()
            .for_each(|pm| pm.match_id = match_id);
    }

    database
        .create_games(games.into_iter().flatten().collect())
        .await?;
    database
        .create_player_matches(
            &player_matches
                .into_iter()
                .flatten()
                .collect::<Vec<PlayerMatch>>(),
        )
        .await?;

    Ok(())
}

fn get_winner(play: &ktool::Play) -> Winner {
    match play.winner {
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
    }
}

pub fn parse(buffer: &[u8]) -> Result<ktool::Tournament> {
    Ok(serde_json::from_slice(buffer)?)
}
