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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_players: Option<RegisteredPlayers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary_license_people: Option<TemporaryLicensePeople>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_to_update: Option<MembersToUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_need_regularization: Option<MembersNeedRegularization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tournaments: Option<Tournaments>,
    #[serde(rename = "$unflatten=retroCompatibilityFastBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retro_compatibility_fast_build: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Uids")]
    pub uids: Option<Uids>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_people: Option<FederationPeople>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub itsf_people: Option<ItsfPeople>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rankings: Option<Rankings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tournament_templates: Option<TournamentTemplates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries_and_lang: Option<CountriesAndLang>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_types: Option<TableTypes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_categories: Option<RankingCategories>,
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
    pub no_license: Option<String>,
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
    #[serde(rename = "$unflatten=minSeries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_series: Option<u32>,
    #[serde(rename = "$unflatten=laterality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub laterality: Option<String>,
    #[serde(rename = "$unflatten=position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "$unflatten=originClub")]
    pub origin_club: String,
    #[serde(rename = "$unflatten=clubsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clubs_list: Option<String>,
    #[serde(rename = "$unflatten=favoriteTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite_table: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    #[serde(rename = "$unflatten=deathDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub death_date: Option<String>,
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
    pub no_license: String,
    #[serde(rename = "$unflatten=federationCountry")]
    pub federation_country: String,
    #[serde(rename = "$unflatten=isAmateurLicense")]
    pub is_amateur_license: bool,
    #[serde(rename = "$unflatten=licenseStatus")]
    pub license_status: String,
    #[serde(rename = "$unflatten=licenseExpiration")]
    #[serde(default)]
    #[serde(with = "crate::serde::option_fast_date_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_expiration: Option<PrimitiveDateTime>,
    #[serde(rename = "$unflatten=licenseDateOrigin")]
    #[serde(default)]
    #[serde(with = "crate::serde::option_fast_date_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_date_origin: Option<PrimitiveDateTime>,
    #[serde(rename = "$unflatten=licenseDateRenewal")]
    #[serde(default)]
    #[serde(with = "crate::serde::option_fast_date_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_date_renewal: Option<PrimitiveDateTime>,
    #[serde(rename = "$unflatten=isLicensePrinted")]
    pub is_license_printed: bool,
    #[serde(rename = "$unflatten=nationalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_code: Option<String>,
    #[serde(rename = "$unflatten=factualLicense")]
    pub factual_license: bool,
    #[serde(rename = "$unflatten=freeLicense")]
    pub free_license: bool,
    #[serde(rename = "$unflatten=regularizationRegistrationDate")]
    #[serde(default)]
    #[serde(with = "crate::serde::option_fast_date_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regularization_registration_date: Option<PrimitiveDateTime>,
    #[serde(rename = "$unflatten=regularizationTournamentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regularization_tournament_id: Option<u32>,
    #[serde(rename = "$unflatten=regularizationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regularization_status: Option<String>,
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Uids {
    pub uid_federation: UidFederation,
    pub uid_licenses: UidLicenses,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UidFederation {
    pub league: Vec<League>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct League {
    #[serde(rename = "$unflatten=noLeague")]
    pub no_league: u32,
    #[serde(rename = "$unflatten=prevId")]
    pub prev_id: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UidLicenses {
    pub country: Vec<UidLicensesCountry>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UidLicensesCountry {
    #[serde(rename = "$unflatten=countryCode")]
    pub country_code: u32,
    #[serde(rename = "$unflatten=prevId")]
    pub prev_id: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FederationPeople {
    pub ffft_league: Vec<FfftLeague>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FfftLeague {
    #[serde(rename = "$unflatten=noLeague")]
    pub no_league: u32,
    #[serde(rename = "$unflatten=name")]
    pub name: String,
    pub ffft_club: Vec<FfftClub>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FfftClub {
    #[serde(rename = "$unflatten=noClub")]
    pub no_club: u32,
    #[serde(rename = "$unflatten=name")]
    pub name: String,
    #[serde(rename = "$unflatten=clubAbbreviation")]
    pub club_abbreviation: String,
    #[serde(rename = "$unflatten=phoneNumber")]
    pub phone_number: String,
    #[serde(rename = "$unflatten=faxNumber")]
    pub fax_number: String,
    #[serde(rename = "$unflatten=email")]
    pub email: String,
    #[serde(rename = "$unflatten=webSiteAddress")]
    pub web_site_address: String,
    #[serde(rename = "$unflatten=officeAddress")]
    pub office_address: String,
    #[serde(rename = "$unflatten=practiceAddress")]
    pub practice_address: String,
    #[serde(rename = "$unflatten=officeCity")]
    pub office_city: String,
    #[serde(rename = "$unflatten=practiceCity")]
    pub practice_city: String,
    #[serde(rename = "$unflatten=officeZipCode")]
    pub office_zip_code: String,
    #[serde(rename = "$unflatten=practiceZipCode")]
    pub practice_zip_code: String,
    #[serde(rename = "$unflatten=practiceInfos")]
    pub practice_infos: String,
    #[serde(rename = "$unflatten=dateValidated")]
    #[serde(default)]
    #[serde(with = "crate::serde::option_fast_date_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_validated: Option<PrimitiveDateTime>,
    #[serde(default)]
    pub ffft_member: Vec<FfftMember>,
    #[serde(rename = "$unflatten=contactName")]
    pub contact_name: String,
    #[serde(rename = "$unflatten=logoId")]
    pub logo_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FfftMember {
    #[serde(rename = "$unflatten=licDateValidated")]
    #[serde(with = "crate::serde::option_fast_date_time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lic_date_validated: Option<PrimitiveDateTime>,
    #[serde(rename = "$unflatten=certificationInsuranceOption")]
    pub certification_insurance_option: u32,
    pub itsf_member: ItsfMember,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItsfPeople {
    pub itsf_member: Vec<ItsfMember>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Rankings;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TournamentTemplates {
    pub tournament_template: Vec<TournamentTemplate>,
    pub tourn_templ_ranking_config: Vec<TournTemplRankingConfig>,
    pub tourn_templ_packages: Vec<TournTemplPackages>,
    pub competition_template: Vec<CompetitionTemplate>,
    pub points_templates: Vec<PointsTemplates>,
    pub points_template_details: Vec<PointsTemplateDetails>,
    pub competition_template_points: Vec<CompetitionTemplatePoints>,
    pub comp_templ_pricing: Vec<CompTemplPricing>,
    pub comp_templ_dotations: Vec<CompTemplDotations>,
    pub comp_templ_phases: Vec<CompTemplPhases>,
    pub comp_templ_formation_config: Vec<CompTemplFormationConfig>,
    pub comp_templ_packages: Vec<CompTemplPackages>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TournamentTemplate {
    #[serde(rename = "$unflatten=id")]
    pub id: u32,
    #[serde(rename = "$unflatten=name")]
    pub name: String,
    #[serde(rename = "$unflatten=tournamentType")]
    pub tournament_type: String,
    #[serde(rename = "$unflatten=tournamentFee1")]
    pub tournament_fee1: String,
    #[serde(rename = "$unflatten=tournamentFee2")]
    pub tournament_fee2: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TournTemplRankingConfig {
    #[serde(rename = "$unflatten=id")]
    pub id: u32,
    #[serde(rename = "$unflatten=tournamentTemplateId")]
    pub tournament_template_id: u32,
    #[serde(rename = "$unflatten=rankingCategoryId")]
    pub ranking_category_id: u32,
    #[serde(rename = "$unflatten=orderNumber")]
    pub order_number: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TournTemplPackages {
    #[serde(rename = "$unflatten=id")]
    pub id: u32,
    #[serde(rename = "$unflatten=tournamentTemplateId")]
    pub tournament_template_id: u32,
    #[serde(rename = "$unflatten=packName")]
    pub pack_name: String,
    #[serde(rename = "$unflatten=packPrice")]
    pub pack_price: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompetitionTemplate {
    #[serde(rename = "$unflatten=id")]
    pub id: u32,
    #[serde(rename = "$unflatten=tournamentTemplateId")]
    pub tournament_template_id: u32,
    #[serde(rename = "$unflatten=type")]
    pub r#type: String,
    #[serde(rename = "$unflatten=tableType")]
    pub table_type: String,
    #[serde(rename = "$unflatten=mastersNr")]
    pub masters_nr: u32,
    #[serde(rename = "$unflatten=mastersNrPercentage")]
    pub masters_nr_percentage: bool,
    #[serde(rename = "$unflatten=division")]
    pub division: u32,
    #[serde(rename = "$unflatten=isMixed")]
    pub is_mixed: bool,
    #[serde(rename = "$unflatten=hasTeams")]
    pub has_teams: bool,
    #[serde(rename = "$unflatten=isStatsCounted")]
    pub is_stats_counted: bool,
    #[serde(rename = "$unflatten=isAmateurLicenseAuthorized")]
    pub is_amateur_license_authorized: bool,
    #[serde(rename = "$unflatten=name")]
    pub name: String,
    #[serde(rename = "$unflatten=includeFee")]
    pub include_fee: u32,
    #[serde(rename = "$unflatten=competitionFee")]
    pub competition_fee: String,
    #[serde(rename = "$unflatten=isFactualLicenseAuthorized")]
    pub is_factual_license_authorized: bool,
    #[serde(rename = "$unflatten=countForRanking")]
    pub count_for_ranking: bool,
    #[serde(rename = "$unflatten=mastersPlayQualification")]
    pub masters_play_qualification: bool,
    #[serde(rename = "$unflatten=sex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
    #[serde(rename = "$unflatten=category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "$unflatten=disabledOnly")]
    pub disabled_only: bool,
    #[serde(rename = "$unflatten=authorizedSeries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_series: Option<String>,
    #[serde(rename = "$unflatten=authPlayer")]
    pub auth_player: String,
    #[serde(rename = "$unflatten=startDay")]
    pub start_day: u32,
    #[serde(rename = "$unflatten=linkToPhaseId")]
    pub link_to_phase_id: u32,
    #[serde(rename = "$unflatten=phaseOrder")]
    pub phase_order: u32,
    #[serde(rename = "$unflatten=rankingSystem")]
    pub ranking_system: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PointsTemplates {
    #[serde(rename = "$unflatten=id")]
    pub id: u32,
    #[serde(rename = "$unflatten=name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PointsTemplateDetails {
    #[serde(rename = "$unflatten=id")]
    pub id: u32,
    #[serde(rename = "$unflatten=pointsTemplateId")]
    pub points_template_id: u32,
    #[serde(rename = "$unflatten=rank")]
    pub rank: u32,
    #[serde(rename = "$unflatten=points")]
    pub points: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompetitionTemplatePoints {
    #[serde(rename = "$unflatten=id")]
    pub id: u32,
    #[serde(rename = "$unflatten=templateId")]
    pub template_id: u32,
    #[serde(rename = "$unflatten=pointsTemplateId")]
    pub points_template_id: u32,
    #[serde(rename = "$unflatten=isMaster")]
    pub is_master: bool,
    #[serde(rename = "$unflatten=minPlayersNr")]
    pub min_players_nr: u32,
    #[serde(rename = "$unflatten=rankingCategoryId")]
    pub ranking_category_id: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompTemplPricing {
    #[serde(rename = "$unflatten=id")]
    pub id: u32,
    #[serde(rename = "$unflatten=templateId")]
    pub template_id: u32,
    #[serde(rename = "$unflatten=playerPrice")]
    pub player_price: String,
    #[serde(rename = "$unflatten=playerCategory")]
    pub player_category: String,
    #[serde(rename = "$unflatten=playerSeries")]
    pub player_series: u32,
    #[serde(rename = "$unflatten=feminine")]
    pub feminine: bool,
    #[serde(rename = "$unflatten=disabled")]
    pub disabled: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompTemplDotations {
    #[serde(rename = "$unflatten=id")]
    pub id: u32,
    #[serde(rename = "$unflatten=templateId")]
    pub template_id: u32,
    #[serde(rename = "$unflatten=rank")]
    pub rank: u32,
    #[serde(rename = "$unflatten=dotationDescription")]
    pub dotation_description: String,
    #[serde(rename = "$unflatten=dotationValue")]
    pub dotation_value: String,
    #[serde(rename = "$unflatten=dotationPercent")]
    pub dotation_percent: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompTemplPhases {
    #[serde(rename = "$unflatten=id")]
    pub id: u32,
    #[serde(rename = "$unflatten=templateId")]
    pub template_id: u32,
    #[serde(rename = "$unflatten=phaseType")]
    pub phase_type: String,
    #[serde(rename = "$unflatten=phaseOrder")]
    pub phase_order: u32,
    #[serde(rename = "$unflatten=isMaster")]
    pub is_master: bool,
    #[serde(rename = "$unflatten=isQualif")]
    pub is_qualif: bool,
    #[serde(rename = "$unflatten=winPoint")]
    pub win_point: u32,
    #[serde(rename = "$unflatten=twoPtsDiff")]
    pub two_pts_diff: bool,
    #[serde(rename = "$unflatten=winGameNr")]
    pub win_game_nr: u32,
    #[serde(rename = "$unflatten=winningGames")]
    pub winning_games: bool,
    #[serde(rename = "$unflatten=gamesNumber")]
    pub games_number: u32,
    #[serde(rename = "$unflatten=beginTime")]
    #[serde(default)]
    #[serde(with = "crate::serde::option_fast_date_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_time: Option<PrimitiveDateTime>,
    #[serde(rename = "$unflatten=matchDuration")]
    pub match_duration: u32,
    #[serde(rename = "$unflatten=matchsTeamNr")]
    pub matchs_team_nr: u32,
    #[serde(rename = "$unflatten=treeDepth")]
    pub tree_depth: u32,
    #[serde(rename = "$unflatten=thirdPlace")]
    pub third_place: bool,
    #[serde(rename = "$unflatten=qualifNumber")]
    pub qualif_number: u32,
    #[serde(rename = "$unflatten=qualifPercent")]
    pub qualif_percent: u32,
    #[serde(rename = "$unflatten=deadPoint")]
    pub dead_point: u32,
    #[serde(rename = "$unflatten=masterRandom")]
    pub master_random: bool,
    #[serde(rename = "$unflatten=forfeitPoint")]
    pub forfeit_point: u32,
    #[serde(rename = "$unflatten=pauseTime")]
    #[serde(default)]
    #[serde(with = "crate::serde::option_fast_date_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_time: Option<PrimitiveDateTime>,
    #[serde(rename = "$unflatten=pauseDuration")]
    pub pause_duration: u32,
    #[serde(rename = "$unflatten=lbWinPoint")]
    pub lb_win_point: u32,
    #[serde(rename = "$unflatten=lbTwoPtsDiff")]
    pub lb_two_pts_diff: bool,
    #[serde(rename = "$unflatten=lbGamesNumber")]
    pub lb_games_number: u32,
    #[serde(rename = "$unflatten=lbWinGameNr")]
    pub lb_win_game_nr: u32,
    #[serde(rename = "$unflatten=lbDeadPoint")]
    pub lb_dead_point: u32,
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
    #[serde(rename = "$unflatten=seriesConstraint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_constraint: Option<String>,
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
    #[serde(rename = "$unflatten=twoLeggedTie")]
    pub two_legged_tie: bool,
    #[serde(rename = "$unflatten=pointsPerElement")]
    pub points_per_element: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompTemplFormationConfig {
    #[serde(rename = "$unflatten=id")]
    pub id: u32,
    #[serde(rename = "$unflatten=templateId")]
    pub template_id: u32,
    #[serde(rename = "$unflatten=formationType")]
    pub formation_type: String,
    #[serde(rename = "$unflatten=elementsMap")]
    pub elements_map: String,
    #[serde(rename = "$unflatten=reserveNumber")]
    pub reserve_number: u32,
    #[serde(rename = "$unflatten=extraMatchType")]
    pub extra_match_type: String,
    #[serde(rename = "$unflatten=maxDistinctPlayers")]
    pub max_distinct_players: u32,
    #[serde(rename = "$unflatten=dynamicFormation")]
    pub dynamic_formation: bool,
    #[serde(rename = "$unflatten=replacementDuringMatch")]
    pub replacement_during_match: bool,
    #[serde(rename = "$unflatten=formationTypeWithoutRestriction")]
    pub formation_type_without_restriction: bool,
    #[serde(rename = "$unflatten=distinctSinglesNumber")]
    pub distinct_singles_number: u32,
    #[serde(rename = "$unflatten=distinctDoublesNumber")]
    pub distinct_doubles_number: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompTemplPackages {
    #[serde(rename = "$unflatten=id")]
    pub id: u32,
    #[serde(rename = "$unflatten=templateId")]
    pub template_id: u32,
    #[serde(rename = "$unflatten=tournamentTemplatePackageId")]
    pub tournament_template_package_id: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CountriesAndLang {
    pub countries: Countries,
    pub countries_lang: CountriesLang,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Countries {
    pub country: Vec<Country>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    #[serde(rename = "$unflatten=numericCode")]
    pub numeric_code: u32,
    #[serde(rename = "$unflatten=countryEN")]
    pub country_en: String,
    #[serde(rename = "$unflatten=countryFR")]
    pub country_fr: String,
    #[serde(rename = "$unflatten=iso3Code")]
    pub iso3_code: String,
    #[serde(rename = "$unflatten=iso2Code")]
    pub iso2_code: String,
    #[serde(rename = "$unflatten=priorityDisplay")]
    pub priority_display: u32,
    #[serde(rename = "$unflatten=phonePrefix")]
    pub phone_prefix: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CountriesLang {
    pub country_lang: Vec<CountryLang>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CountryLang {
    #[serde(rename = "$unflatten=iso3CountryCode")]
    pub iso3_country_code: String,
    #[serde(rename = "$unflatten=langCode")]
    pub lang_code: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TableTypes {
    pub table_type: Vec<TableType>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TableType {
    #[serde(rename = "$unflatten=id")]
    pub id: u32,
    #[serde(rename = "$unflatten=code")]
    pub code: String,
    #[serde(rename = "$unflatten=abbrev")]
    pub abbrev: String,
    #[serde(rename = "$unflatten=name")]
    pub name: String,
    #[serde(rename = "$unflatten=scope")]
    pub scope: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RankingCategories {
    pub ranking_category: Vec<RankingCategory>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RankingCategory {
    pub id: u32,
    #[serde(rename = "$unflatten=name")]
    pub name: String,
    #[serde(rename = "$unflatten=excelTemplateFilename")]
    pub excel_template_filename: String,
    #[serde(rename = "RankingCategorySeason")]
    pub ranking_category_season: Vec<RankingCategorySeason>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RankingCategorySeason {
    pub id: u32,
    #[serde(rename = "$unflatten=rankingCategoryId")]
    pub ranking_category_id: u32,
    #[serde(rename = "$unflatten=seasonName")]
    pub season_name: String,
    #[serde(rename = "$unflatten=beginSeasonDate")]
    #[serde(with = "crate::serde::fast_date")]
    pub begin_season_date: Date,
    #[serde(rename = "$unflatten=endSeasonDate")]
    #[serde(with = "crate::serde::fast_date")]
    pub end_season_date: Date,
    #[serde(rename = "$unflatten=bestResultsNumber")]
    pub best_results_number: u32,
}
