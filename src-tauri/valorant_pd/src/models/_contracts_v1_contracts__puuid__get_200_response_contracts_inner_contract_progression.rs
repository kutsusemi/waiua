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
pub struct ContractsV1ContractsPuuidGet200ResponseContractsInnerContractProgression {
    #[serde(rename = "TotalProgressionEarned")]
    pub total_progression_earned: f64,
    #[serde(rename = "TotalProgressionEarnedVersion")]
    pub total_progression_earned_version: f64,
    #[serde(rename = "HighestRewardedLevel")]
    pub highest_rewarded_level: std::collections::HashMap<String, models::ContractsV1ContractsPuuidGet200ResponseContractsInnerContractProgressionHighestRewardedLevelValue>,
}

impl ContractsV1ContractsPuuidGet200ResponseContractsInnerContractProgression {
    pub fn new(
        total_progression_earned: f64,
        total_progression_earned_version: f64,
        highest_rewarded_level: std::collections::HashMap<String, models::ContractsV1ContractsPuuidGet200ResponseContractsInnerContractProgressionHighestRewardedLevelValue>,
    ) -> ContractsV1ContractsPuuidGet200ResponseContractsInnerContractProgression {
        ContractsV1ContractsPuuidGet200ResponseContractsInnerContractProgression {
            total_progression_earned,
            total_progression_earned_version,
            highest_rewarded_level,
        }
    }
}
