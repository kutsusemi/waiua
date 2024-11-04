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
pub struct StoreV1EntitlementsPuuidItemTypeIdGet200ResponseEntitlementsByTypesInnerEntitlementsInner {
    /// UUID
    #[serde(rename = "TypeID")]
    pub type_id: String,
    /// Item ID
    #[serde(rename = "ItemID")]
    pub item_id: String,
    /// UUID
    #[serde(rename = "InstanceID", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

impl StoreV1EntitlementsPuuidItemTypeIdGet200ResponseEntitlementsByTypesInnerEntitlementsInner {
    pub fn new(type_id: String, item_id: String) -> StoreV1EntitlementsPuuidItemTypeIdGet200ResponseEntitlementsByTypesInnerEntitlementsInner {
        StoreV1EntitlementsPuuidItemTypeIdGet200ResponseEntitlementsByTypesInnerEntitlementsInner {
            type_id,
            item_id,
            instance_id: None,
        }
    }
}
