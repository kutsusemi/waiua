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
pub struct ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerXpGrantsModifierModifiersInner
{
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "Name")]
    pub name: Name,
    #[serde(rename = "BaseOnly")]
    pub base_only: bool,
}

impl ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerXpGrantsModifierModifiersInner {
    pub fn new(
        value: f64,
        name: Name,
        base_only: bool,
    ) -> ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerXpGrantsModifierModifiersInner
    {
        ContractsV1ContractsPuuidGet200ResponseProcessedMatchesInnerXpGrantsModifierModifiersInner {
            value,
            name,
            base_only,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "RESTRICTIONS_XP")]
    RestrictionsXp,
    #[serde(rename = "PREMIUM_CONTRACT_XP")]
    PremiumContractXp,
}

impl Default for Name {
    fn default() -> Name {
        Self::RestrictionsXp
    }
}
