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
pub struct StoreV1OffersGet200ResponseOffersInnerRewardsInner {
    /// Item Type ID
    #[serde(rename = "ItemTypeID")]
    pub item_type_id: String,
    /// Item ID
    #[serde(rename = "ItemID")]
    pub item_id: String,
    #[serde(rename = "Quantity")]
    pub quantity: f64,
}

impl StoreV1OffersGet200ResponseOffersInnerRewardsInner {
    pub fn new(
        item_type_id: String,
        item_id: String,
        quantity: f64,
    ) -> StoreV1OffersGet200ResponseOffersInnerRewardsInner {
        StoreV1OffersGet200ResponseOffersInnerRewardsInner {
            item_type_id,
            item_id,
            quantity,
        }
    }
}
