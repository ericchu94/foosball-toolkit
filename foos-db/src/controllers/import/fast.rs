use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    io::{BufReader, Cursor, Read, Seek},
};

use actix_web::web::Data;
use fast::TeamMatch;
use log::{debug, warn};
use time::format_description::well_known::Rfc3339;
use time_tz::PrimitiveDateTimeExt;
use zip::ZipArchive;

use crate::{database::Database, models::*};

use super::{
    ImportError::{MissingField, MissingPlayer},
    ImportResult,
};

fn get_standard_player_by_player_id(f: &fast::Fast, id: u32) -> Option<Player> {
    f.registered_players
        .iter()
        .flat_map(|rp| rp.player_infos.iter())
        .flat_map(|pi| &pi.player)
        .chain(
            f.temporary_license_people
                .iter()
                .flat_map(|tlp| tlp.itsf_member.iter())
                .map(|im| &im.federation_member.player),
        )
        .find(|p| p.id == id)
        .map(|p| Player {
            id: 0,
            first_name: p.person.first_name.clone(),
            last_name: p.person.last_name.clone(),
        })
}

async fn get_players_by_licenses(
    database: Data<Database>,
    licenses: &[String],
) -> ImportResult<HashMap<String, Player>> {
    Ok(database
        .get_fast_players_by_licenses(licenses)
        .await?
        .into_iter()
        .map(|fp| {
            (
                fp.license,
                Player {
                    first_name: fp.first_name,
                    last_name: fp.last_name,
                    ..Default::default()
                },
            )
        })
        .collect())
}

fn get_license_by_player_id(f: &fast::Fast, id: u32) -> Option<String> {
    f.registered_players
        .iter()
        .flat_map(|rp| rp.player_infos.iter())
        .filter(|pi| pi.player_id.is_some() && pi.no_license.is_some())
        .find(|pi| pi.player_id.unwrap() == id)
        .and_then(|pi| pi.no_license.clone())
}

async fn get_players_by_player_ids(
    database: Data<Database>,
    f: &fast::Fast,
    ids: &[u32],
) -> ImportResult<HashMap<u32, Player>> {
    let (mut players, missing_ids) = ids
        .iter()
        .map(|&id| (id, get_standard_player_by_player_id(f, id)))
        .fold((HashMap::new(), vec![]), |(mut m, mut v), (id, o)| {
            match o {
                Some(p) => {
                    m.insert(id, p);
                }
                None => v.push(id),
            }
            (m, v)
        });

    let map = missing_ids
        .into_iter()
        .map(|id| {
            get_license_by_player_id(f, id)
                .map(|license| (id, license))
                .ok_or(MissingPlayer(id))
        })
        .collect::<ImportResult<HashMap<u32, String>>>()?;

    let licenses = map.values().cloned().collect::<Vec<String>>();

    let mut licensed_players = get_players_by_licenses(database, &licenses).await?;

    for r in map.into_iter().map(|(id, license)| {
        let player = licensed_players.remove(&license).ok_or(MissingPlayer(id))?;
        ImportResult::Ok((id, player))
    }) {
        let (id, player) = r?;
        players.insert(id, player);
    }

    Ok(players)
}

pub async fn import_fast(database: Data<Database>, f: fast::Fast) -> ImportResult<()> {
    let t = &f
        .tournaments
        .as_ref()
        .ok_or(MissingField("tournaments"))?
        .tournament;

    let tournament = Tournament {
        name: t.name.clone(),
        source: String::from("fast"),
        ..Default::default()
    };
    let tournament = database.get_or_create_tournament(tournament).await?;

    let mut team_matches = t
        .competition
        .iter()
        .flat_map(|c| c.phase.iter())
        .flat_map(|p| p.team_match.iter())
        .filter(|tm| tm.team1_id.is_some() && tm.team2_id.is_some())
        .collect::<Vec<&fast::TeamMatch>>();
    team_matches.sort_by_key(|tm| tm.schedule_end);

    let get_players_from_team = |id: u32| {
        let team = &t
            .competition
            .iter()
            .flat_map(|c| c.competition_team.iter())
            .find(|ct| ct.id == id)
            .unwrap()
            .team;

        let mut v = vec![team.player1_id];
        v.extend(team.player2_id.iter());

        v
    };

    let get_winner = |tm: &TeamMatch| {
        let (a, b) =
            tm.game
                .iter()
                .fold((0, 0), |acc, g| match g.score_team1.cmp(&g.score_team2) {
                    Ordering::Greater => (acc.0 + 2, acc.1),
                    Ordering::Less => (acc.0, acc.1 + 2),
                    Ordering::Equal => (acc.0 + 1, acc.1 + 1),
                });

        match a.cmp(&b) {
            Ordering::Greater => Winner::Team1,
            Ordering::Less => Winner::Team2,
            Ordering::Equal => Winner::Draw,
        }
    };

    let get_games = |tm: &TeamMatch| {
        let mut games = tm.game.clone();
        games.sort_by_key(|g| g.game_number);
        games
            .into_iter()
            .map(|g| {
                let score1 = g.score_team1 as i32;
                let score2 = g.score_team2 as i32;

                Game {
                    score1,
                    score2,
                    ..Game::default()
                }
            })
            .collect::<Vec<Game>>()
    };

    let mut all_games = vec![];

    let player_ids = team_matches
        .iter()
        .flat_map(|tm| {
            let t1 = *tm.team1_id.as_ref().unwrap();
            let t2 = *tm.team2_id.as_ref().unwrap();
            let players1 = get_players_from_team(t1);
            let players2 = get_players_from_team(t2);
            players1.into_iter().chain(players2.into_iter())
        })
        .collect::<Vec<u32>>();

    let players = get_players_by_player_ids(database.clone(), &f, &player_ids).await?;
    database
        .create_players(&players.values().cloned().collect::<Vec<Player>>())
        .await?;
    let names = players
        .values()
        .map(|p| (p.first_name.as_str(), p.last_name.as_str()))
        .collect::<HashSet<(&str, &str)>>();
    let mut db_players = database.get_players_by_names(names).await?;
    let players = players
        .into_iter()
        .map(|(id, p)| {
            let p = db_players
                .remove(&(p.first_name, p.last_name))
                .ok_or(MissingPlayer(id))?;
            ImportResult::Ok((id, p))
        })
        .collect::<ImportResult<HashMap<u32, Player>>>()?;

    for tm in team_matches {
        let t1 = *tm.team1_id.as_ref().unwrap();
        let t2 = *tm.team2_id.as_ref().unwrap();
        let players1 = get_players_from_team(t1);
        let players1 = players1
            .into_iter()
            .map(|id| &players[&id])
            .collect::<Vec<&Player>>();
        let players2 = get_players_from_team(t2);
        let players2 = players2
            .into_iter()
            .map(|id| &players[&id])
            .collect::<Vec<&Player>>();
        let winner = get_winner(tm);

        let mut games = get_games(tm);

        let tz = time_tz::timezones::get_by_name(&t.time_zone).unwrap();

        let r#match = Match {
            id: 0,
            tournament_id: Some(tournament.id),
            timestamp: tm.schedule_end.assume_timezone(tz).unwrap(),
            winner,
        };

        // Draw disallowed for 0-0 and BoX
        let skip = winner == Winner::Draw
            && (games[0].score1 == 0 && games[0].score2 == 0 || games.len() > 1);

        if skip {
            warn!(
                "Skipped TeamMatch: {}: {:?} {:?} vs {:?}. Winner: {:?}, Games: {:?}",
                tm.id,
                r#match.timestamp.format(&Rfc3339).unwrap(),
                players1
                    .iter()
                    .map(|p| &p.first_name)
                    .collect::<Vec<&String>>(),
                players2
                    .iter()
                    .map(|p| &p.first_name)
                    .collect::<Vec<&String>>(),
                winner,
                games.iter().map(|g| g.to_string()).collect::<Vec<String>>()
            );
            continue;
        }

        debug!(
            "{:?} {:?} vs {:?}. Winner: {:?}, Games: {:?}",
            r#match.timestamp.format(&Rfc3339).unwrap(),
            players1
                .iter()
                .map(|p| &p.first_name)
                .collect::<Vec<&String>>(),
            players2
                .iter()
                .map(|p| &p.first_name)
                .collect::<Vec<&String>>(),
            winner,
            games.iter().map(|g| g.to_string()).collect::<Vec<String>>()
        );

        let r#match = database
            .create_match_and_player_matches(r#match, players1, players2)
            .await?;

        for g in games.iter_mut() {
            g.match_id = r#match.id;
        }

        all_games.append(&mut games);
    }

    database.create_games(all_games).await?;

    Ok(())
}

pub fn parse<'a>(buffer: &'a mut &'a [u8]) -> ImportResult<fast::Fast> {
    let mut archive = read_archive(buffer)?;
    let file = archive.by_index(0)?;

    let reader = BufReader::new(file);

    Ok(quick_xml::de::from_reader(reader)?)
}

fn read_archive<'a>(buffer: &'a mut &'a [u8]) -> ImportResult<ZipArchive<impl Read + Seek + 'a>> {
    let reader = Cursor::new(buffer);
    let archive = ZipArchive::new(reader)?;

    Ok(archive)
}

pub async fn import_fast_init(database: Data<Database>, f: fast::Fast) -> ImportResult<()> {
    let federation_members = f
        .itsf_people
        .iter()
        .flat_map(|p| p.itsf_member.iter())
        .chain(
            f.federation_people
                .iter()
                .flat_map(|p| p.ffft_league.iter())
                .flat_map(|l| l.ffft_club.iter())
                .flat_map(|c| c.ffft_member.iter())
                .map(|m| &m.itsf_member),
        )
        .map(|m| &m.federation_member);

    let fast_players = federation_members
        .map(|fm| {
            let license = fm.no_license.clone();
            let first_name = fm.player.person.first_name.clone();
            let last_name = fm.player.person.last_name.clone();

            FastPlayer {
                id: 0,
                license,
                first_name,
                last_name,
            }
        })
        .collect::<Vec<FastPlayer>>();

    database.create_fast_players(fast_players).await?;

    Ok(())
}
