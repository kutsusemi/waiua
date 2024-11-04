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
pub struct ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerRewardSchedule {
    /// UUID
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Prerequisites", deserialize_with = "Option::deserialize")]
    pub prerequisites: Option<serde_json::Value>,
    #[serde(rename = "RewardsPerLevel", deserialize_with = "Option::deserialize")]
    pub rewards_per_level: Option<Vec<models::ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerRewardScheduleRewardsPerLevelInner>>,
}

impl ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerRewardSchedule {
    pub fn new(id: String, name: String, prerequisites: Option<serde_json::Value>, rewards_per_level: Option<Vec<models::ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerRewardScheduleRewardsPerLevelInner>>) -> ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerRewardSchedule {
        ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerRewardSchedule {
            id,
            name,
            prerequisites,
            rewards_per_level,
        }
    }
}
