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
pub struct ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInner {
    /// UUID
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Item")]
    pub item: Box<models::ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerItem>,
    #[serde(rename = "RequiredEntitlement")]
    pub required_entitlement: Box<models::ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerItem>,
    #[serde(rename = "ProgressionSchedule")]
    pub progression_schedule: Box<models::ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerProgressionSchedule>,
    #[serde(rename = "RewardSchedule")]
    pub reward_schedule: Box<models::ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerRewardSchedule>,
    #[serde(rename = "Sidegrades", deserialize_with = "Option::deserialize")]
    pub sidegrades: Option<Vec<models::ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerSidegradesInner>>,
}

impl ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInner {
    pub fn new(id: String, item: models::ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerItem, required_entitlement: models::ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerItem, progression_schedule: models::ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerProgressionSchedule, reward_schedule: models::ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerRewardSchedule, sidegrades: Option<Vec<models::ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerSidegradesInner>>) -> ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInner {
        ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInner {
            id,
            item: Box::new(item),
            required_entitlement: Box::new(required_entitlement),
            progression_schedule: Box::new(progression_schedule),
            reward_schedule: Box::new(reward_schedule),
            sidegrades,
        }
    }
}

