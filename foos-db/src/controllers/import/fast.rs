use std::io::{BufReader, Cursor, Read, Seek};

use actix_web::web::Data;
use fast::TeamMatch;
use time_tz::PrimitiveDateTimeExt;
use zip::ZipArchive;

use crate::{database::Database, models::*};

use super::{ImportError::MissingField, ImportResult};

pub async fn import_fast(database: Data<Database>, f: fast::Fast) -> ImportResult<()> {
    let t = f.tournaments.ok_or(MissingField("tournaments"))?.tournament;

    let tournament = Tournament {
        name: t.name,
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

        let players = f
            .registered_players
            .iter()
            .flat_map(|rp| rp.player_infos.iter())
            .flat_map(|pi| &pi.player)
            .chain(
                f.temporary_license_people
                    .iter()
                    .flat_map(|tlp| tlp.itsf_member.iter())
                    .map(|im| &im.federation_member.player),
            )
            .filter(|p| {
                p.id == team.player1_id
                    || if let Some(&player2_id) = team.player2_id.as_ref() {
                        p.id == player2_id
                    } else {
                        false
                    }
            })
            .map(|p| Player {
                id: 0,
                first_name: p.person.first_name.clone(),
                last_name: p.person.last_name.clone(),
            })
            .collect::<Vec<Player>>();

        players
    };

    let get_winner = |tm: &TeamMatch| {
        let (a, b) = tm.game.iter().fold((0, 0), |acc, g| {
            if g.score_team1 > g.score_team2 {
                (acc.0 + 2, acc.1)
            } else if g.score_team1 < g.score_team2 {
                (acc.0, acc.1 + 2)
            } else {
                (acc.0 + 1, acc.1 + 1)
            }
        });
        if a > b {
            Winner::Team1
        } else if b > a {
            Winner::Team2
        } else {
            Winner::Draw
        }
    };

    for tm in team_matches {
        let t1 = *tm.team1_id.as_ref().unwrap();
        let t2 = *tm.team2_id.as_ref().unwrap();
        let players1 = get_players_from_team(t1);
        let players2 = get_players_from_team(t2);
        let winner = get_winner(tm);

        let tz = time_tz::timezones::get_by_name(&t.time_zone).unwrap();

        let r#match = Match {
            id: 0,
            tournament_id: Some(tournament.id),
            timestamp: tm.schedule_end.assume_timezone(tz).unwrap(),
            winner,
        };

        println!(
            "{:?} {:?} vs {:?}. Winner: {:?}",
            r#match.timestamp, players1, players2, winner
        );

        database
            .create_match_and_players(r#match, players1, players2)
            .await?;
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

    for fm in federation_members {
        let license = fm.no_license.clone();
        let first_name = fm.player.person.first_name.clone();
        let last_name = fm.player.person.last_name.clone();

        let fast_player = FastPlayer {
            id: 0,
            license,
            first_name,
            last_name,
        };

        database.create_fast_player(fast_player).await?;
    }

    Ok(())
}
