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
pub struct MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsAdaptiveBots {
    #[serde(rename = "adaptiveBotAverageDurationMillisAllAttempts")]
    pub adaptive_bot_average_duration_millis_all_attempts: AdaptiveBotAverageDurationMillisAllAttempts,
    #[serde(rename = "adaptiveBotAverageDurationMillisFirstAttempt")]
    pub adaptive_bot_average_duration_millis_first_attempt: AdaptiveBotAverageDurationMillisFirstAttempt,
    #[serde(rename = "killDetailsFirstAttempt", deserialize_with = "Option::deserialize")]
    pub kill_details_first_attempt: Option<serde_json::Value>,
    #[serde(rename = "idleTimeMillis")]
    pub idle_time_millis: IdleTimeMillis,
    #[serde(rename = "objectiveCompleteTimeMillis")]
    pub objective_complete_time_millis: ObjectiveCompleteTimeMillis,
}

impl MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsAdaptiveBots {
    pub fn new(adaptive_bot_average_duration_millis_all_attempts: AdaptiveBotAverageDurationMillisAllAttempts, adaptive_bot_average_duration_millis_first_attempt: AdaptiveBotAverageDurationMillisFirstAttempt, kill_details_first_attempt: Option<serde_json::Value>, idle_time_millis: IdleTimeMillis, objective_complete_time_millis: ObjectiveCompleteTimeMillis) -> MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsAdaptiveBots {
        MatchDetailsV1MatchesMatchIdGet200ResponsePlayersInnerNewPlayerExperienceDetailsAdaptiveBots {
            adaptive_bot_average_duration_millis_all_attempts,
            adaptive_bot_average_duration_millis_first_attempt,
            kill_details_first_attempt,
            idle_time_millis,
            objective_complete_time_millis,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AdaptiveBotAverageDurationMillisAllAttempts {
    #[serde(rename = "0")]
    Variant0,
}

impl Default for AdaptiveBotAverageDurationMillisAllAttempts {
    fn default() -> AdaptiveBotAverageDurationMillisAllAttempts {
        Self::Variant0
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AdaptiveBotAverageDurationMillisFirstAttempt {
    #[serde(rename = "0")]
    Variant0,
}

impl Default for AdaptiveBotAverageDurationMillisFirstAttempt {
    fn default() -> AdaptiveBotAverageDurationMillisFirstAttempt {
        Self::Variant0
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
