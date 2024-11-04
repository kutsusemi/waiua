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
pub struct MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerStatsAbilityCasts {
    #[serde(rename = "grenadeCasts")]
    pub grenade_casts: f64,
    #[serde(rename = "ability1Casts")]
    pub ability1_casts: f64,
    #[serde(rename = "ability2Casts")]
    pub ability2_casts: f64,
    #[serde(rename = "ultimateCasts")]
    pub ultimate_casts: f64,
}

impl MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerStatsAbilityCasts {
    pub fn new(grenade_casts: f64, ability1_casts: f64, ability2_casts: f64, ultimate_casts: f64) -> MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerStatsAbilityCasts {
        MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerStatsAbilityCasts {
            grenade_casts,
            ability1_casts,
            ability2_casts,
            ultimate_casts,
        }
    }
}
