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
pub struct StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundleItemsInnerItem {
    /// Item Type ID
    #[serde(rename = "ItemTypeID")]
    pub item_type_id: String,
    /// Item ID
    #[serde(rename = "ItemID")]
    pub item_id: String,
    #[serde(rename = "Amount")]
    pub amount: f64,
}

impl StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundleItemsInnerItem {
    pub fn new(item_type_id: String, item_id: String, amount: f64) -> StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundleItemsInnerItem {
        StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundleItemsInnerItem {
            item_type_id,
            item_id,
            amount,
        }
    }
}
