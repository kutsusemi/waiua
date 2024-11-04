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
pub struct CoreGameV1MatchesCurrentGameMatchIdLoadoutsGet200Response {
    #[serde(rename = "Loadouts")]
    pub loadouts: Vec<models::CoreGameV1MatchesCurrentGameMatchIdLoadoutsGet200ResponseLoadoutsInner>,
}

impl CoreGameV1MatchesCurrentGameMatchIdLoadoutsGet200Response {
    pub fn new(loadouts: Vec<models::CoreGameV1MatchesCurrentGameMatchIdLoadoutsGet200ResponseLoadoutsInner>) -> CoreGameV1MatchesCurrentGameMatchIdLoadoutsGet200Response {
        CoreGameV1MatchesCurrentGameMatchIdLoadoutsGet200Response {
            loadouts,
        }
    }
}
