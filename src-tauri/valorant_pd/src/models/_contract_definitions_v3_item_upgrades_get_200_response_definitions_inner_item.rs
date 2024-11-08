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
pub struct ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerItem {
    /// Item Type ID
    #[serde(rename = "ItemTypeID")]
    pub item_type_id: String,
    /// Item ID
    #[serde(rename = "ItemID")]
    pub item_id: String,
}

impl ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerItem {
    pub fn new(
        item_type_id: String,
        item_id: String,
    ) -> ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerItem {
        ContractDefinitionsV3ItemUpgradesGet200ResponseDefinitionsInnerItem {
            item_type_id,
            item_id,
        }
    }
}
