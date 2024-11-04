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
pub struct MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetails {
    #[serde(rename = "basicMovement")]
    pub basic_movement: Box<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsBasicMovement>,
    #[serde(rename = "basicGunSkill")]
    pub basic_gun_skill: Box<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsBasicMovement>,
    #[serde(rename = "adaptiveBots")]
    pub adaptive_bots: Box<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsAdaptiveBots>,
    #[serde(rename = "ability")]
    pub ability: Box<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsBasicMovement>,
    #[serde(rename = "bombPlant")]
    pub bomb_plant: Box<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsBasicMovement>,
    #[serde(rename = "defendBombSite")]
    pub defend_bomb_site: Box<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsDefendBombSite>,
    #[serde(rename = "settingStatus")]
    pub setting_status: Box<models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsSettingStatus>,
    #[serde(rename = "versionString")]
    pub version_string: VersionString,
}

impl MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetails {
    pub fn new(basic_movement: models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsBasicMovement, basic_gun_skill: models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsBasicMovement, adaptive_bots: models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsAdaptiveBots, ability: models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsBasicMovement, bomb_plant: models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsBasicMovement, defend_bomb_site: models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsDefendBombSite, setting_status: models::MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsSettingStatus, version_string: VersionString) -> MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetails {
        MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetails {
            basic_movement: Box::new(basic_movement),
            basic_gun_skill: Box::new(basic_gun_skill),
            adaptive_bots: Box::new(adaptive_bots),
            ability: Box::new(ability),
            bomb_plant: Box::new(bomb_plant),
            defend_bomb_site: Box::new(defend_bomb_site),
            setting_status: Box::new(setting_status),
            version_string,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VersionString {
    #[serde(rename = "")]
    Empty,
}

impl Default for VersionString {
    fn default() -> VersionString {
        Self::Empty
    }
}
