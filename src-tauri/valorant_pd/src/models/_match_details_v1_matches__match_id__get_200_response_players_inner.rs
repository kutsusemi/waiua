/*
 * Valorant API
 *
 * Valorant API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInner {
    /// Player UUID
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "tagLine")]
    pub tag_line: String,
    #[serde(rename = "platformInfo")]
    pub platform_info: Box<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerPlatformInfo>,
    #[serde(rename = "teamId")]
    pub team_id: Box<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerTeamId>,
    /// Party ID
    #[serde(rename = "partyId")]
    pub party_id: String,
    /// Character ID
    #[serde(rename = "characterId")]
    pub character_id: String,
    #[serde(rename = "stats", deserialize_with = "Option::deserialize")]
    pub stats: Option<Box<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerStats>>,
    #[serde(rename = "roundDamage", deserialize_with = "Option::deserialize")]
    pub round_damage: Option<Vec<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerRoundDamageInner>>,
    #[serde(rename = "competitiveTier")]
    pub competitive_tier: f64,
    #[serde(rename = "isObserver")]
    pub is_observer: bool,
    /// Card ID
    #[serde(rename = "playerCard")]
    pub player_card: String,
    /// Title ID
    #[serde(rename = "playerTitle")]
    pub player_title: String,
    #[serde(rename = "preferredLevelBorder", skip_serializing_if = "Option::is_none")]
    pub preferred_level_border: Option<Box<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerPreferredLevelBorder>>,
    #[serde(rename = "accountLevel")]
    pub account_level: f64,
    #[serde(rename = "sessionPlaytimeMinutes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub session_playtime_minutes: Option<Option<f64>>,
    #[serde(rename = "xpModifications", skip_serializing_if = "Option::is_none")]
    pub xp_modifications: Option<Vec<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerXpModificationsInner>>,
    #[serde(rename = "behaviorFactors", skip_serializing_if = "Option::is_none")]
    pub behavior_factors: Option<Box<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerBehaviorFactors>>,
    #[serde(rename = "newPlayerExperienceDetails", skip_serializing_if = "Option::is_none")]
    pub new_player_experience_details: Option<Box<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetails>>,
}

impl MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInner {
    pub fn new(subject: String, game_name: String, tag_line: String, platform_info: models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerPlatformInfo, team_id: models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerTeamId, party_id: String, character_id: String, stats: Option<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerStats>, round_damage: Option<Vec<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerRoundDamageInner>>, competitive_tier: f64, is_observer: bool, player_card: String, player_title: String, account_level: f64) -> MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInner {
        MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInner {
            subject,
            game_name,
            tag_line,
            platform_info: Box::new(platform_info),
            team_id: Box::new(team_id),
            party_id,
            character_id,
            stats: if let Some(x) = stats {Some(Box::new(x))} else {None},
            round_damage,
            competitive_tier,
            is_observer,
            player_card,
            player_title,
            preferred_level_border: None,
            account_level,
            session_playtime_minutes: None,
            xp_modifications: None,
            behavior_factors: None,
            new_player_experience_details: None,
        }
    }
}
