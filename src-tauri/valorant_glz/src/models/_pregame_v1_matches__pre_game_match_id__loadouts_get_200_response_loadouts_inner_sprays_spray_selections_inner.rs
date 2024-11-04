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
pub struct PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerSpraysSpraySelectionsInner {
    /// UUID
    #[serde(rename = "SocketID")]
    pub socket_id: String,
    /// UUID
    #[serde(rename = "SprayID")]
    pub spray_id: String,
    /// UUID
    #[serde(rename = "LevelID")]
    pub level_id: String,
}

impl PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerSpraysSpraySelectionsInner {
    pub fn new(socket_id: String, spray_id: String, level_id: String) -> PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerSpraysSpraySelectionsInner {
        PregameV1MatchesPreGameMatchIdLoadoutsGet200ResponseLoadoutsInnerSpraysSpraySelectionsInner {
            socket_id,
            spray_id,
            level_id,
        }
    }
}

