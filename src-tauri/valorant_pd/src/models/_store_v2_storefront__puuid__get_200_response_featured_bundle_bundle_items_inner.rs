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
pub struct StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundleItemsInner {
    #[serde(rename = "Item")]
    pub item: Box<models::StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundleItemsInnerItem>,
    #[serde(rename = "BasePrice")]
    pub base_price: f64,
    /// Currency ID
    #[serde(rename = "CurrencyID")]
    pub currency_id: String,
    #[serde(rename = "DiscountPercent")]
    pub discount_percent: f64,
    #[serde(rename = "DiscountedPrice")]
    pub discounted_price: f64,
    #[serde(rename = "IsPromoItem")]
    pub is_promo_item: bool,
}

impl StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundleItemsInner {
    pub fn new(item: models::StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundleItemsInnerItem, base_price: f64, currency_id: String, discount_percent: f64, discounted_price: f64, is_promo_item: bool) -> StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundleItemsInner {
        StoreV2StorefrontPuuidGet200ResponseFeaturedBundleBundleItemsInner {
            item: Box::new(item),
            base_price,
            currency_id,
            discount_percent,
            discounted_price,
            is_promo_item,
        }
    }
}
