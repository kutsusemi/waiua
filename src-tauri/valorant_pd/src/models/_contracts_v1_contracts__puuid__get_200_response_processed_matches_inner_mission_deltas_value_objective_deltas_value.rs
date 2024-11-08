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
pub struct ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerMissionDeltasValueObjectiveDeltasValue
{
    /// UUID
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "ProgressBefore")]
    pub progress_before: f64,
    #[serde(rename = "ProgressAfter")]
    pub progress_after: f64,
}

impl ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerMissionDeltasValueObjectiveDeltasValue {
    pub fn new(id: String, progress_before: f64, progress_after: f64) -> ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerMissionDeltasValueObjectiveDeltasValue {
        ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerMissionDeltasValueObjectiveDeltasValue {
            id,
            progress_before,
            progress_after,
        }
    }
}
