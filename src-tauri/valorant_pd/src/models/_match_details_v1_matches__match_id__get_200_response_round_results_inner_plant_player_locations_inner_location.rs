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
pub struct MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlantPlayerLocationsInnerLocation {
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "y")]
    pub y: f64,
}

impl MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlantPlayerLocationsInnerLocation {
    pub fn new(x: f64, y: f64) -> MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlantPlayerLocationsInnerLocation {
        MatchDetailsV1MatchesMatchIdGet200ResponseRoundResultsInnerPlantPlayerLocationsInnerLocation {
            x,
            y,
        }
    }
}
