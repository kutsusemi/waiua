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
pub struct StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundle {
    /// UUID
    #[serde(rename = "ID")]
    pub id: String,
    /// UUID
    #[serde(rename = "DataAssetID")]
    pub data_asset_id: String,
    /// Currency ID
    #[serde(rename = "CurrencyID")]
    pub currency_id: String,
    #[serde(rename = "Items")]
    pub items: Vec<models::StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundleItemsInner>,
    #[serde(rename = "ItemOffers", deserialize_with = "Option::deserialize")]
    pub item_offers: Option<Vec<models::StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundleItemOffersInner>>,
    #[serde(rename = "TotalBaseCost", deserialize_with = "Option::deserialize")]
    pub total_base_cost: Option<std::collections::HashMap<String, f64>>,
    #[serde(rename = "TotalDiscountedCost", deserialize_with = "Option::deserialize")]
    pub total_discounted_cost: Option<std::collections::HashMap<String, f64>>,
    #[serde(rename = "TotalDiscountPercent")]
    pub total_discount_percent: f64,
    #[serde(rename = "DurationRemainingInSeconds")]
    pub duration_remaining_in_seconds: f64,
    #[serde(rename = "WholesaleOnly")]
    pub wholesale_only: bool,
}

impl StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundle {
    pub fn new(id: String, data_asset_id: String, currency_id: String, items: Vec<models::StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundleItemsInner>, item_offers: Option<Vec<models::StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundleItemOffersInner>>, total_base_cost: Option<std::collections::HashMap<String, f64>>, total_discounted_cost: Option<std::collections::HashMap<String, f64>>, total_discount_percent: f64, duration_remaining_in_seconds: f64, wholesale_only: bool) -> StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundle {
        StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundle {
            id,
            data_asset_id,
            currency_id,
            items,
            item_offers,
            total_base_cost,
            total_discounted_cost,
            total_discount_percent,
            duration_remaining_in_seconds,
            wholesale_only,
        }
    }
}
