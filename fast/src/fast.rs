use serde::{Deserialize, Serialize};
use time::{Date, PrimitiveDateTime};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename = "ffft")]
#[serde(rename_all = "camelCase")]
pub struct Fast {
    #[serde(rename = "$unflatten=creationDate")]
    pub creation_date: String,
    #[serde(rename = "$unflatten=fastVersion")]
    pub fast_version: String,
    #[serde(rename = "$unflatten=fastBuild")]
    pub fast_build: u32,
    pub registered_players: RegisteredPlayers,
    pub temporary_license_people: TemporaryLicensePeople,
    pub members_to_update: MembersToUpdate,
    pub members_need_regularization: MembersNeedRegularization,
    pub tournaments: Tournaments,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredPlayers {
    pub player_infos: Vec<PlayerInfos>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfos {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player: Option<Player>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$unflatten=noLicense")]
    pub no_license: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$unflatten=playerId")]
    pub player_id: Option<u32>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    #[serde(rename = "$unflatten=id")]
    pub id: u32,
    #[serde(rename = "$unflatten=nationality")]
    pub nationality: String,
    #[serde(rename = "$unflatten=series")]
    pub series: u32,
    #[serde(rename = "$unflatten=originClub")]
    pub origin_club: String,
    #[serde(rename = "$unflatten=disabled")]
    pub disabled: bool,
    #[serde(rename = "$unflatten=playerCode")]
    pub player_code: String,
    pub person: Person,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "$unflatten=sex")]
    pub sex: String,
    #[serde(rename = "$unflatten=firstName")]
    pub first_name: String,
    #[serde(rename = "$unflatten=lastName")]
    pub last_name: String,
    #[serde(rename = "$unflatten=birthDate")]
    pub birth_date: String,
    #[serde(rename = "$unflatten=photoPath")]
    pub photo_path: String,
    #[serde(rename = "$unflatten=country")]
    pub country: String,
    #[serde(rename = "$unflatten=email")]
    pub email: String,
    #[serde(rename = "$unflatten=mobileNumber")]
    pub mobile_number: String,
    #[serde(rename = "$unflatten=alertsOnEmail")]
    pub alerts_on_email: bool,
    #[serde(rename = "$unflatten=alertsOnPhone")]
    pub alerts_on_phone: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TemporaryLicensePeople {
    pub itsf_member: Vec<ItsfMember>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItsfMember {
    pub federation_member: FederationMember,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FederationMember {
    #[serde(rename = "$unflatten=noLicense")]
    pub no_license: u32,
    #[serde(rename = "$unflatten=federationCountry")]
    pub federation_country: String,
    #[serde(rename = "$unflatten=isAmateurLicense")]
    pub is_amateur_license: bool,
    #[serde(rename = "$unflatten=licenseStatus")]
    pub license_status: String,
    #[serde(rename = "$unflatten=licenseExpiration")]
    #[serde(with = "crate::serde::fast_date_time")]
    pub license_expiration: PrimitiveDateTime,
    #[serde(rename = "$unflatten=licenseDateOrigin")]
    #[serde(with = "crate::serde::fast_date_time")]
    pub license_date_origin: PrimitiveDateTime,
    #[serde(rename = "$unflatten=licenseDateRenewal")]
    #[serde(with = "crate::serde::fast_date_time")]
    pub license_date_renewal: PrimitiveDateTime,
    #[serde(rename = "$unflatten=isLicensePrinted")]
    pub is_license_printed: bool,
    #[serde(rename = "$unflatten=factualLicense")]
    pub factual_license: bool,
    #[serde(rename = "$unflatten=freeLicense")]
    pub free_license: bool,
    pub player: Player,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MembersToUpdate;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MembersNeedRegularization;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tournaments {
    pub tournament: Tournament,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tournament {
    pub id: u32,
    #[serde(rename = "$unflatten=name")]
    pub name: String,
    #[serde(rename = "$unflatten=type")]
    pub r#type: String,
    #[serde(rename = "$unflatten=tournamentAddr")]
    pub tournament_addr: String,
    #[serde(rename = "$unflatten=beginDate")]
    #[serde(with = "crate::serde::fast_date")]
    pub begin_date: Date,
    #[serde(rename = "$unflatten=endDate")]
    #[serde(with = "crate::serde::fast_date")]
    pub end_date: Date,
    #[serde(rename = "$unflatten=status")]
    pub status: String,
    #[serde(rename = "$unflatten=country")]
    pub country: String,
    #[serde(rename = "$unflatten=countryName")]
    pub country_name: String,
    #[serde(rename = "$unflatten=email")]
    pub email: String,
    #[serde(rename = "$unflatten=phoneNumber")]
    pub phone_number: String,
    #[serde(rename = "$unflatten=faxNumber")]
    pub fax_number: String,
    #[serde(rename = "$unflatten=webSite")]
    pub web_site: String,
    #[serde(rename = "$unflatten=managerName")]
    pub manager_name: String,
    #[serde(rename = "$unflatten=isValidated")]
    pub is_validated: bool,
    #[serde(rename = "$unflatten=tournamentFee1")]
    pub tournament_fee1: String,
    #[serde(rename = "$unflatten=tournamentFee2")]
    pub tournament_fee2: String,
    #[serde(rename = "$unflatten=smsProviderLogin")]
    pub sms_provider_login: String,
    #[serde(rename = "$unflatten=smsProviderPassword")]
    pub sms_provider_password: String,
    #[serde(rename = "$unflatten=scoringUrl")]
    pub scoring_url: String,
    #[serde(rename = "$unflatten=scoringToken")]
    pub scoring_token: String,
    #[serde(rename = "$unflatten=scoringKey")]
    pub scoring_key: String,
    #[serde(rename = "$unflatten=liveEnabled")]
    pub live_enabled: bool,
    #[serde(rename = "$unflatten=onlineRegisterUntilDate")]
    pub online_register_until_date: String,
    #[serde(rename = "$unflatten=originalId")]
    pub original_id: u32,
    #[serde(rename = "$unflatten=unactivateBarCodeSecurity")]
    pub unactivate_bar_code_security: bool,
    #[serde(rename = "$unflatten=timeZone")]
    pub time_zone: String,
    pub table: Vec<Table>,
    pub competition: Vec<Competition>,
    pub ranking_configuration: RankingConfiguration,
    pub public_display: Vec<PublicDisplay>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    #[serde(rename = "$unflatten=id")]
    pub id: u32,
    #[serde(rename = "$unflatten=tableNumber")]
    pub table_number: u32,
    #[serde(rename = "$unflatten=tableType")]
    pub table_type: String,
    #[serde(rename = "$unflatten=isMasterTable")]
    pub is_master_table: bool,
    #[serde(rename = "$unflatten=status")]
    pub status: String,
    #[serde(rename = "$unflatten=mapX")]
    pub map_x: u32,
    #[serde(rename = "$unflatten=mapY")]
    pub map_y: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Competition {
    pub id: u32,
    #[serde(rename = "$unflatten=type")]
    pub r#type: String,
    #[serde(rename = "$unflatten=noTableMin")]
    pub no_table_min: u32,
    #[serde(rename = "$unflatten=noTableMax")]
    pub no_table_max: u32,
    #[serde(rename = "$unflatten=tableType")]
    pub table_type: String,
    #[serde(rename = "$unflatten=beginDate")]
    #[serde(with = "crate::serde::fast_date_time")]
    pub begin_date: PrimitiveDateTime,
    #[serde(rename = "$unflatten=endDate")]
    #[serde(with = "crate::serde::fast_date_time")]
    pub end_date: PrimitiveDateTime,
    #[serde(rename = "$unflatten=mastersNr")]
    pub masters_nr: u32,
    #[serde(rename = "$unflatten=mastersNrPercentage")]
    pub masters_nr_percentage: bool,
    #[serde(rename = "$unflatten=status")]
    pub status: String,
    #[serde(rename = "$unflatten=sex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
    #[serde(rename = "$unflatten=disabledOnly")]
    pub disabled_only: bool,
    #[serde(rename = "$unflatten=division")]
    pub division: u32,
    #[serde(rename = "$unflatten=isMixed")]
    pub is_mixed: bool,
    #[serde(rename = "$unflatten=hasTeams")]
    pub has_teams: bool,
    #[serde(rename = "$unflatten=isStatsCounted")]
    pub is_stats_counted: bool,
    #[serde(rename = "$unflatten=authPlayer")]
    pub auth_player: String,
    #[serde(rename = "$unflatten=duplicateNumber")]
    pub duplicate_number: u32,
    #[serde(rename = "$unflatten=hasOnlyResults")]
    pub has_only_results: bool,
    #[serde(rename = "$unflatten=isAmateurLicenseAuthorized")]
    pub is_amateur_license_authorized: bool,
    #[serde(rename = "$unflatten=name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "$unflatten=includeFee")]
    pub include_fee: u32,
    #[serde(rename = "$unflatten=activateSMSAlerts")]
    pub activate_smsalerts: bool,
    #[serde(rename = "$unflatten=competitionFee")]
    pub competition_fee: String,
    #[serde(rename = "$unflatten=isFactualLicenseAuthorized")]
    pub is_factual_license_authorized: bool,
    #[serde(rename = "$unflatten=countForRanking")]
    pub count_for_ranking: bool,
    #[serde(rename = "$unflatten=mastersPlayQualification")]
    pub masters_play_qualification: bool,
    #[serde(rename = "$unflatten=linkToPhaseId")]
    pub link_to_phase_id: u32,
    #[serde(rename = "$unflatten=phaseOrder")]
    pub phase_order: u32,
    #[serde(rename = "$unflatten=rankingSystem")]
    pub ranking_system: String,
    #[serde(rename = "$unflatten=originalId")]
    pub original_id: u32,
    #[serde(rename = "$unflatten=pointsTemplateId")]
    pub points_template_id: u32,
    #[serde(rename = "$unflatten=hideTableType")]
    pub hide_table_type: bool,
    pub competition_dotations: CompetitionDotations,
    pub competition_pricing: CompetitionPricing,
    #[serde(default)]
    pub phase: Vec<Phase>,
    pub competition_team: Vec<CompetitionTeam>,
    pub packages: Option<Packages>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompetitionDotations;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompetitionPricing;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompetitionTeam {
    #[serde(rename = "$unflatten=id")]
    pub id: u32,
    pub team: Team,
    #[serde(rename = "$unflatten=noTeam")]
    pub no_team: u32,
    #[serde(rename = "$unflatten=isMaster")]
    pub is_master: bool,
    #[serde(rename = "$unflatten=isValidated")]
    pub is_validated: bool,
    #[serde(rename = "$unflatten=isPlayer1Validated")]
    pub is_player1_validated: bool,
    #[serde(rename = "$unflatten=isPlayer2Validated")]
    pub is_player2_validated: bool,
    #[serde(rename = "$unflatten=isBlank")]
    pub is_blank: bool,
    #[serde(rename = "$unflatten=registerDate")]
    #[serde(with = "crate::serde::fast_date_time")]
    pub register_date: PrimitiveDateTime,
    #[serde(rename = "$unflatten=isProtected")]
    pub is_protected: bool,
    #[serde(rename = "$unflatten=player1PaymentMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player1_payment_mode: Option<String>,
    #[serde(rename = "$unflatten=player2PaymentMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player2_payment_mode: Option<String>,
    #[serde(rename = "$unflatten=player1Series")]
    pub player1_series: u32,
    #[serde(rename = "$unflatten=player2Series")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player2_series: Option<u32>,
    #[serde(rename = "$unflatten=groupNumber")]
    pub group_number: u32,
    #[serde(rename = "$unflatten=registerUserLogins")]
    pub register_user_logins: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    #[serde(rename = "$unflatten=player1Id")]
    pub player1_id: u32,
    #[serde(rename = "$unflatten=player2Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player2_id: Option<u32>,
    #[serde(rename = "$unflatten=forwardPlayerNumber")]
    pub forward_player_number: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Packages;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Phase {
    pub id: u32,
    #[serde(rename = "$unflatten=noTableMin")]
    pub no_table_min: u32,
    #[serde(rename = "$unflatten=noTableMax")]
    pub no_table_max: u32,
    #[serde(rename = "$unflatten=phaseType")]
    pub phase_type: String,
    #[serde(rename = "$unflatten=phaseOrder")]
    pub phase_order: u32,
    #[serde(rename = "$unflatten=isMaster")]
    pub is_master: bool,
    #[serde(rename = "$unflatten=isQualification")]
    pub is_qualification: bool,
    #[serde(rename = "$unflatten=winPoint")]
    pub win_point: u32,
    #[serde(rename = "$unflatten=hasTwoPointsDiff")]
    pub has_two_points_diff: bool,
    #[serde(rename = "$unflatten=winGameNumber")]
    pub win_game_number: u32,
    #[serde(rename = "$unflatten=winningGames")]
    pub winning_games: bool,
    #[serde(rename = "$unflatten=gamesNumber")]
    pub games_number: u32,
    #[serde(rename = "$unflatten=matchDuration")]
    pub match_duration: u32,
    #[serde(rename = "$unflatten=matchesNrPerTeam")]
    pub matches_nr_per_team: u32,
    #[serde(rename = "$unflatten=treeDepth")]
    pub tree_depth: u32,
    #[serde(rename = "$unflatten=hasThirdPlace")]
    pub has_third_place: bool,
    #[serde(rename = "$unflatten=deadPoint")]
    pub dead_point: u32,
    #[serde(rename = "$unflatten=qualificationNumber")]
    pub qualification_number: u32,
    #[serde(rename = "$unflatten=qualificationPercent")]
    pub qualification_percent: u32,
    #[serde(rename = "$unflatten=isMasterRandomPosition")]
    pub is_master_random_position: bool,
    #[serde(rename = "$unflatten=forfeitPoint")]
    pub forfeit_point: u32,
    #[serde(rename = "$unflatten=pauseDuration")]
    pub pause_duration: u32,
    #[serde(rename = "$unflatten=loserBracketWinPoint")]
    pub loser_bracket_win_point: u32,
    #[serde(rename = "$unflatten=hasLoserBracketTwoPointsDiff")]
    pub has_loser_bracket_two_points_diff: bool,
    #[serde(rename = "$unflatten=loserBracketWinGameNumber")]
    pub loser_bracket_win_game_number: u32,
    #[serde(rename = "$unflatten=loserBracketGamesNumber")]
    pub loser_bracket_games_number: u32,
    #[serde(rename = "$unflatten=loserBracketDeadPoint")]
    pub loser_bracket_dead_point: u32,
    #[serde(rename = "$unflatten=extraMatchWinPoint")]
    pub extra_match_win_point: u32,
    #[serde(rename = "$unflatten=extraMatchWinGameNr")]
    pub extra_match_win_game_nr: u32,
    #[serde(rename = "$unflatten=extraMatchTwoPtsDiff")]
    pub extra_match_two_pts_diff: bool,
    #[serde(rename = "$unflatten=extraMatchDeadPoint")]
    pub extra_match_dead_point: u32,
    #[serde(rename = "$unflatten=groupNumber")]
    pub group_number: u32,
    #[serde(rename = "$unflatten=mastersNumberPerGroup")]
    pub masters_number_per_group: u32,
    #[serde(rename = "$unflatten=globalWinMatch")]
    pub global_win_match: bool,
    #[serde(rename = "$unflatten=goalAverageWinMatch")]
    pub goal_average_win_match: bool,
    #[serde(rename = "$unflatten=useScore")]
    pub use_score: bool,
    #[serde(rename = "$unflatten=maxSeriesConstraintNumber")]
    pub max_series_constraint_number: u32,
    #[serde(rename = "$unflatten=refereedMatches")]
    pub refereed_matches: bool,
    #[serde(rename = "$unflatten=allowTie")]
    pub allow_tie: bool,
    #[serde(rename = "$unflatten=waitForEndOfRoundForMatchesCreation")]
    pub wait_for_end_of_round_for_matches_creation: bool,
    #[serde(rename = "$unflatten=firstRoundProtection")]
    pub first_round_protection: bool,
    #[serde(rename = "$unflatten=firstRoundProtectionPercent")]
    pub first_round_protection_percent: u32,
    #[serde(rename = "$unflatten=secondRoundProtection")]
    pub second_round_protection: bool,
    #[serde(rename = "$unflatten=secondRoundProtectionPercent")]
    pub second_round_protection_percent: u32,
    #[serde(rename = "$unflatten=mustHaveTwoRoundsGap")]
    pub must_have_two_rounds_gap: bool,
    #[serde(rename = "$unflatten=allowScoreTie")]
    pub allow_score_tie: bool,
    #[serde(rename = "$unflatten=activateSMSAlerts")]
    pub activate_smsalerts: bool,
    #[serde(rename = "$unflatten=originalId")]
    pub original_id: u32,
    #[serde(rename = "$unflatten=twoLeggedTie")]
    pub two_legged_tie: bool,
    #[serde(rename = "$unflatten=pointsPerElement")]
    pub points_per_element: u32,
    pub phase_ranking: PhaseRanking,
    #[serde(default)]
    pub team_match: Vec<TeamMatch>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PhaseRanking {
    pub ranking: Vec<Ranking>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ranking {
    #[serde(rename = "$unflatten=groupNumber")]
    pub group_number: u32,
    #[serde(rename = "$unflatten=rank")]
    pub rank: u32,
    pub definitive_phase_opponent_ranking: DefinitivePhaseOpponentRanking,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefinitivePhaseOpponentRanking {
    #[serde(rename = "$unflatten=teamId")]
    pub team_id: u32,
    #[serde(rename = "$unflatten=relativeRank")]
    pub relative_rank: u32,
    #[serde(rename = "$unflatten=qualified")]
    pub qualified: bool,
    #[serde(rename = "$unflatten=bestPlaceRank")]
    pub best_place_rank: u32,
    pub points: Points,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Points {
    pub ranking_category_id: u32,
    #[serde(rename = "$value")]
    pub value: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamMatch {
    pub id: u32,
    #[serde(rename = "$unflatten=principalTableId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_table_id: Option<u32>,
    #[serde(rename = "$unflatten=status")]
    pub status: String,
    #[serde(rename = "$unflatten=team1Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team1_id: Option<u32>,
    #[serde(rename = "$unflatten=team2Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team2_id: Option<u32>,
    #[serde(rename = "$unflatten=scheduleStart")]
    #[serde(with = "crate::serde::fast_date_time")]
    pub schedule_start: PrimitiveDateTime,
    #[serde(rename = "$unflatten=effectiveStart")]
    #[serde(with = "crate::serde::fast_date_time")]
    pub effective_start: PrimitiveDateTime,
    #[serde(rename = "$unflatten=scheduleEnd")]
    #[serde(with = "crate::serde::fast_date_time")]
    pub schedule_end: PrimitiveDateTime,
    #[serde(rename = "$unflatten=matchDepth")]
    pub match_depth: i32,
    #[serde(rename = "$unflatten=nodeRank")]
    pub node_rank: i32,
    #[serde(rename = "$unflatten=isWinnerBracket")]
    pub is_winner_bracket: bool,
    #[serde(rename = "$unflatten=matchNumber")]
    pub match_number: i32,
    pub game: Vec<Game>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    #[serde(rename = "$unflatten=scoreTeam1")]
    pub score_team1: u32,
    #[serde(rename = "$unflatten=scoreTeam2")]
    pub score_team2: u32,
    #[serde(rename = "$unflatten=gameNumber")]
    pub game_number: u32,
    #[serde(rename = "$unflatten=elementNumber")]
    pub element_number: u32,
    pub game_statistics: GameStatistics,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameStatistics;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RankingConfiguration;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublicDisplay {
    #[serde(rename = "$unflatten=displayId")]
    pub display_id: i32,
    #[serde(rename = "$unflatten=displayType")]
    pub display_type: u32,
    #[serde(rename = "$unflatten=objectId")]
    pub object_id: u32,
    #[serde(rename = "$unflatten=orderBy")]
    pub order_by: u32,
    #[serde(rename = "$unflatten=mode")]
    pub mode: u32,
    #[serde(rename = "$unflatten=description")]
    pub description: String,
    #[serde(rename = "$unflatten=linesPerPage")]
    pub lines_per_page: u32,
    #[serde(rename = "$unflatten=secPerPage")]
    pub sec_per_page: u32,
    #[serde(rename = "$unflatten=resolutionType")]
    pub resolution_type: String,
}
