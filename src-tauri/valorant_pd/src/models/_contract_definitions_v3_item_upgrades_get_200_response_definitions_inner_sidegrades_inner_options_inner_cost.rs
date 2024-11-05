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
pub struct ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerSidegradesInnerOptionsInnerCost {
    #[serde(rename = "WalletCosts")]
    pub wallet_costs: Vec<models::ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerSidegradesInnerOptionsInnerCostWalletCostsInner>,
}

impl
    ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerSidegradesInnerOptionsInnerCost
{
    pub fn new(wallet_costs: Vec<models::ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerSidegradesInnerOptionsInnerCostWalletCostsInner>) -> ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerSidegradesInnerOptionsInnerCost{
        ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerSidegradesInnerOptionsInnerCost {
            wallet_costs,
        }
    }
}
