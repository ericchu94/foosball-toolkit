use std::{collections::HashMap, iter};

use actix_multipart::Multipart;
use actix_web::{
    post,
    web::{self, Data, ServiceConfig},
    HttpResponse, Responder, Result,
};
use futures::{stream, StreamExt};
use time::OffsetDateTime;

use crate::{
    database::Database,
    models::{Tournament, Winner, Match, Player},
};

#[post("")]
async fn import(payload: Multipart, database: Data<Database>) -> Result<impl Responder> {
    println!("Test");

    let map = payload
        .map(Result::unwrap)
        .then(|field| async {
            let name = field.name().to_owned();
            let value = field
                .map(Result::unwrap)
                .flat_map(stream::iter)
                .collect::<Vec<u8>>()
                .await;
            (name, value)
        })
        .collect::<HashMap<String, Vec<u8>>>()
        .await;

    let file = &map["file"];

    let kt = parse(file)?;

    import_kt(database, kt).await?;

    Ok(HttpResponse::Ok())
}

async fn import_kt(database: Data<Database>, kt: ktool::Tournament) -> Result<()> {
    let tournament = Tournament {
        name: kt.name,
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
        kt
            .players
            .iter()
            .find(|player| player.id == player_id)
            .unwrap()
    };

    let get_players_from_team = |team_id: &str| {
        let team = kt
            .teams
            .iter()
            .find(|team| team.id == team_id)
            .unwrap();
        team.players
            .iter()
            .map(|player| get_player(&player.id).name.clone())
            .collect::<Vec<String>>()
    };

    for play in plays {
        let t1 = play.team1.as_ref().unwrap();
        let t2 = play.team2.as_ref().unwrap();
        let players1 = get_players_from_team(&t1.id);
        let players2 = get_players_from_team(&t2.id);
        let winner = get_winner(play);

        let r#match = Match {
            id: 0,
            tournament_id: Some(tournament.id),
            timestamp: OffsetDateTime::from_unix_timestamp(play.time_end.unwrap() as i64 / 1000),
            winner,
        };
        
        println!(
            "{:?} {:?} vs {:?}. Winner: {:?}",
            play.time_end, players1, players2, winner
        );

        let team1 = players1.into_iter().map(|p| Player {
            id: 0,
            first_name: p,
            last_name: String::new(),
        }).collect::<Vec<Player>>();

        let team2 = players2.into_iter().map(|p| Player {
            id: 0,
            first_name: p,
            last_name: String::new(),
        }).collect::<Vec<Player>>();

        database.create_match_and_players(r#match, team1, team2).await?;

    }

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

fn parse(buffer: &[u8]) -> Result<ktool::Tournament> {
    Ok(serde_json::from_slice(buffer)?)
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/import").service(import));
}
