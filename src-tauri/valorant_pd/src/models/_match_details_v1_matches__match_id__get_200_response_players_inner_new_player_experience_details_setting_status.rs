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
pub struct MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsSettingStatus {
    #[serde(rename = "isMouseSensitivityDefault")]
    pub is_mouse_sensitivity_default: bool,
    #[serde(rename = "isCrosshairDefault")]
    pub is_crosshair_default: bool,
}

impl MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsSettingStatus {
    pub fn new(is_mouse_sensitivity_default: bool, is_crosshair_default: bool) -> MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsSettingStatus {
        MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsSettingStatus {
            is_mouse_sensitivity_default,
            is_crosshair_default,
        }
    }
}
