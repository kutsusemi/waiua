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
pub struct ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerMissionDeltasValue {
    /// UUID
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Objectives")]
    pub objectives: std::collections::HashMap<String, f64>,
    #[serde(rename = "ObjectiveDeltas")]
    pub objective_deltas: std::collections::HashMap<String, models::ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerMissionDeltasValueObjectiveDeltasValue>,
}

impl ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerMissionDeltasValue {
    pub fn new(id: String, objectives: std::collections::HashMap<String, f64>, objective_deltas: std::collections::HashMap<String, models::ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerMissionDeltasValueObjectiveDeltasValue>) -> ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerMissionDeltasValue {
        ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerMissionDeltasValue {
            id,
            objectives,
            objective_deltas,
        }
    }
}
