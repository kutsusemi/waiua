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
pub struct ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInner {
    /// Match ID
    #[serde(rename = "ID")]
    pub id: String,
    /// Milliseconds since epoch
    #[serde(rename = "StartTime")]
    pub start_time: f64,
    #[serde(rename = "XPGrants", deserialize_with = "Option::deserialize")]
    pub xp_grants:
        Option<Box<models::ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerXpGrants>>,
    #[serde(rename = "RewardGrants", deserialize_with = "Option::deserialize")]
    pub reward_grants: Option<serde_json::Value>,
    #[serde(rename = "MissionDeltas", deserialize_with = "Option::deserialize")]
    pub mission_deltas: Option<
        std::collections::HashMap<
            String,
            models::ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerMissionDeltasValue,
        >,
    >,
    #[serde(rename = "ContractDeltas", deserialize_with = "Option::deserialize")]
    pub contract_deltas: Option<
        std::collections::HashMap<
            String,
            models::ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerContractDeltasValue,
        >,
    >,
    #[serde(rename = "CouldProgressMissions")]
    pub could_progress_missions: bool,
}

impl ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInner {
    pub fn new(
        id: String,
        start_time: f64,
        xp_grants: Option<
            models::ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerXpGrants,
        >,
        reward_grants: Option<serde_json::Value>,
        mission_deltas: Option<std::collections::HashMap<String, models::ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerMissionDeltasValue>>,
        contract_deltas: Option<std::collections::HashMap<String, models::ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerContractDeltasValue>>,
        could_progress_missions: bool,
    ) -> ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInner {
        ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInner {
            id,
            start_time,
            xp_grants: if let Some(x) = xp_grants {
                Some(Box::new(x))
            } else {
                None
            },
            reward_grants,
            mission_deltas,
            contract_deltas,
            could_progress_missions,
        }
    }
}
