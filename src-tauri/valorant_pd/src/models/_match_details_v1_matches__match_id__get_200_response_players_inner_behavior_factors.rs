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
pub struct MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerBehaviorFactors {
    #[serde(rename = "afkRounds")]
    pub afk_rounds: f64,
    /// Float value of unknown significance. Possibly used to quantify how much the player was in the way of their teammates?
    #[serde(rename = "collisions", skip_serializing_if = "Option::is_none")]
    pub collisions: Option<f64>,
    #[serde(rename = "commsRatingRecovery")]
    pub comms_rating_recovery: f64,
    #[serde(rename = "damageParticipationOutgoing")]
    pub damage_participation_outgoing: f64,
    #[serde(rename = "friendlyFireIncoming", skip_serializing_if = "Option::is_none")]
    pub friendly_fire_incoming: Option<f64>,
    #[serde(rename = "friendlyFireOutgoing", skip_serializing_if = "Option::is_none")]
    pub friendly_fire_outgoing: Option<f64>,
    #[serde(rename = "mouseMovement", skip_serializing_if = "Option::is_none")]
    pub mouse_movement: Option<f64>,
    #[serde(rename = "stayedInSpawnRounds", skip_serializing_if = "Option::is_none")]
    pub stayed_in_spawn_rounds: Option<f64>,
}

impl MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerBehaviorFactors {
    pub fn new(afk_rounds: f64, comms_rating_recovery: f64, damage_participation_outgoing: f64) -> MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerBehaviorFactors {
        MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerBehaviorFactors {
            afk_rounds,
            collisions: None,
            comms_rating_recovery,
            damage_participation_outgoing,
            friendly_fire_incoming: None,
            friendly_fire_outgoing: None,
            mouse_movement: None,
            stayed_in_spawn_rounds: None,
        }
    }
}
