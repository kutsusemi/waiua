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
pub struct ContractsV1ContractsPuuidGet200ResponseContractsInnerContractProgressionHighestRewardedLevelValue
{
    #[serde(rename = "Amount")]
    pub amount: f64,
    #[serde(rename = "Version")]
    pub version: f64,
}

impl ContractsV1ContractsPuuidGet200ResponseContractsInnerContractProgressionHighestRewardedLevelValue {
    pub fn new(amount: f64, version: f64) -> ContractsV1ContractsPuuidGet200ResponseContractsInnerContractProgressionHighestRewardedLevelValue {
        ContractsV1ContractsPuuidGet200ResponseContractsInnerContractProgressionHighestRewardedLevelValue {
            amount,
            version,
        }
    }
}
