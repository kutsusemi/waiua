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
pub struct StoreV1EntitlementsPuuidItemTypeIdGet200Response {
    #[serde(rename = "EntitlementsByTypes")]
    pub entitlements_by_types:
        Vec<models::StoreV1EntitlementsPuuidItemTypeIdGet200ResponseEntitlementsByTypesInner>,
}

impl StoreV1EntitlementsPuuidItemTypeIdGet200Response {
    pub fn new(
        entitlements_by_types: Vec<
            models::StoreV1EntitlementsPuuidItemTypeIdGet200ResponseEntitlementsByTypesInner,
        >,
    ) -> StoreV1EntitlementsPuuidItemTypeIdGet200Response {
        StoreV1EntitlementsPuuidItemTypeIdGet200Response {
            entitlements_by_types,
        }
    }
}
