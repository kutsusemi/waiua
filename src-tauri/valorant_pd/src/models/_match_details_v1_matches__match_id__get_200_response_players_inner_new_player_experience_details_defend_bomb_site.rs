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
pub struct MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsDefendBombSite {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "idleTimeMillis")]
    pub idle_time_millis: IdleTimeMillis,
    #[serde(rename = "objectiveCompleteTimeMillis")]
    pub objective_complete_time_millis: ObjectiveCompleteTimeMillis,
}

impl MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsDefendBombSite {
    pub fn new(success: bool, idle_time_millis: IdleTimeMillis, objective_complete_time_millis: ObjectiveCompleteTimeMillis) -> MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsDefendBombSite {
        MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsDefendBombSite {
            success,
            idle_time_millis,
            objective_complete_time_millis,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IdleTimeMillis {
    #[serde(rename = "0")]
    Variant0,
}

impl Default for IdleTimeMillis {
    fn default() -> IdleTimeMillis {
        Self::Variant0
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ObjectiveCompleteTimeMillis {
    #[serde(rename = "0")]
    Variant0,
}

impl Default for ObjectiveCompleteTimeMillis {
    fn default() -> ObjectiveCompleteTimeMillis {
        Self::Variant0
    }
}
