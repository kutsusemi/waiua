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
pub struct PartiesV1PartiesPartyIdGet200ResponseMatchmakingData {
    /// Queue ID
    #[serde(rename = "QueueID")]
    pub queue_id: String,
    #[serde(rename = "PreferredGamePods")]
    pub preferred_game_pods: Vec<String>,
    #[serde(rename = "SkillDisparityRRPenalty")]
    pub skill_disparity_rr_penalty: f64,
}

impl PartiesV1PartiesPartyIdGet200ResponseMatchmakingData {
    pub fn new(queue_id: String, preferred_game_pods: Vec<String>, skill_disparity_rr_penalty: f64) -> PartiesV1PartiesPartyIdGet200ResponseMatchmakingData {
        PartiesV1PartiesPartyIdGet200ResponseMatchmakingData {
            queue_id,
            preferred_game_pods,
            skill_disparity_rr_penalty,
        }
    }
}
