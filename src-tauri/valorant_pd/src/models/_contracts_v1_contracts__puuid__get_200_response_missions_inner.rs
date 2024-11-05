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
pub struct ContractsV1ContractsPuuidGet200ResponseMissionsInner {
    /// UUID
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Objectives")]
    pub objectives: std::collections::HashMap<String, f64>,
    #[serde(rename = "Complete")]
    pub complete: bool,
    /// Date in ISO 8601 format
    #[serde(rename = "ExpirationTime")]
    pub expiration_time: String,
}

impl ContractsV1ContractsPuuidGet200ResponseMissionsInner {
    pub fn new(
        id: String,
        objectives: std::collections::HashMap<String, f64>,
        complete: bool,
        expiration_time: String,
    ) -> ContractsV1ContractsPuuidGet200ResponseMissionsInner {
        ContractsV1ContractsPuuidGet200ResponseMissionsInner {
            id,
            objectives,
            complete,
            expiration_time,
        }
    }
}
