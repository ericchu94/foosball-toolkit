use std::collections::HashMap;

use actix_web::{
    get,
    web::{self, Data, Json, Query, ServiceConfig},
    Responder, Result,
};
use futures::{stream, StreamExt, TryStreamExt};
use serde::Deserialize;
use time::format_description::well_known::Rfc3339;

use crate::{database::Database, models::*};

#[derive(Deserialize)]
struct MatchQuery {
    limit: Option<i32>,
    offset: Option<i32>,
}

#[get("")]
async fn get_matches(database: Data<Database>, query: Query<MatchQuery>) -> Result<impl Responder> {
    let limit = query.limit.unwrap_or(100);
    let offset = query.offset.unwrap_or(0);
    let matches = database.get_matches(limit, offset).await?;
    Ok(Json(matches))
}

#[get("/pretty")]
async fn get_matches_pretty(database: Data<Database>, query: Query<MatchQuery>) -> Result<impl Responder> {
    let limit = query.limit.unwrap_or(100);
    let offset = query.limit.unwrap_or(0);
    let matches = database.get_matches(limit, offset).await?;

    let strings = stream::iter(matches)
        .then(|m| {
            let database = database.clone();
            async move {
                let player_matches = database.get_player_matches_by_match_id(m.id).await?;
                dbg!(&player_matches);
                dbg!(&m);
                let teams = stream::iter(player_matches)
                    .then(|pm| {
                        let database = database.clone();
                        async move {
                            let player = database.get_player_by_id(pm.player_id).await?;
                            crate::database::Result::Ok((pm.team, player))
                        }
                    })
                    .try_fold(HashMap::new(), |mut acc, (team, player)| async {
                        acc.entry(team).or_insert(vec![]).push(player);
                        Ok(acc)
                    })
                    .await?;

                crate::database::Result::Ok((m, teams))
            }
        })
        .and_then(|(m, teams)| async move {
            dbg!(&teams);
            let team1 = teams[&Team::Team1]
                .iter()
                .map(|p| format!("{} {}", p.first_name, p.last_name))
                .collect::<Vec<String>>()
                .join(" ");
            let team2 = teams[&Team::Team2]
                .iter()
                .map(|p| format!("{} {}", p.first_name, p.last_name))
                .collect::<Vec<String>>()
                .join(" ");

            Ok(format!(
                "{} {}",
                m.timestamp.format(&Rfc3339).unwrap(),
                match m.winner {
                    Winner::Team1 => format!("{team1} beat {team2}"),
                    Winner::Team2 => format!("{team2} beat {team1}"),
                    Winner::None => String::from("None"),
                    Winner::Draw => format!("{team2} drew {team1}"),
                }
            ))
        })
        .try_collect::<Vec<String>>()
        .await?;

    Ok(Json(strings))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/match")
            .service(get_matches)
            .service(get_matches_pretty),
    );
}
