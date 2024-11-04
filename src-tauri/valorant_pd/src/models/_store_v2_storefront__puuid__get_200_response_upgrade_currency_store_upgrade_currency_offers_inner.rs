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
pub struct StoreV2StorefrontPuuidGet200ResponseUpgradeCurrencyStoreUpgradeCurrencyOffersInner {
    /// UUID
    #[serde(rename = "OfferID")]
    pub offer_id: String,
    /// Item ID
    #[serde(rename = "StorefrontItemID")]
    pub storefront_item_id: String,
    #[serde(rename = "Offer")]
    pub offer: Box<models::StoreV1OffersGet200ResponseOffersInner>,
    #[serde(rename = "DiscountedPercent")]
    pub discounted_percent: f64,
}

impl StoreV2StorefrontPuuidGet200ResponseUpgradeCurrencyStoreUpgradeCurrencyOffersInner {
    pub fn new(offer_id: String, storefront_item_id: String, offer: models::StoreV1OffersGet200ResponseOffersInner, discounted_percent: f64) -> StoreV2StorefrontPuuidGet200ResponseUpgradeCurrencyStoreUpgradeCurrencyOffersInner {
        StoreV2StorefrontPuuidGet200ResponseUpgradeCurrencyStoreUpgradeCurrencyOffersInner {
            offer_id,
            storefront_item_id,
            offer: Box::new(offer),
            discounted_percent,
        }
    }
}
