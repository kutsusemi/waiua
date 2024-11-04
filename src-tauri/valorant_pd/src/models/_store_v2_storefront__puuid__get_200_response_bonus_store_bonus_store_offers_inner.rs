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
pub struct StoreV2StorefrontPuuidGet200ResponseBonusStoreBonusStoreOffersInner {
    /// UUID
    #[serde(rename = "BonusOfferID")]
    pub bonus_offer_id: String,
    #[serde(rename = "Offer")]
    pub offer: Box<models::StoreV1OffersGet200ResponseOffersInner>,
    #[serde(rename = "DiscountPercent")]
    pub discount_percent: f64,
    #[serde(rename = "DiscountCosts")]
    pub discount_costs: std::collections::HashMap<String, f64>,
    #[serde(rename = "IsSeen")]
    pub is_seen: bool,
}

impl StoreV2StorefrontPuuidGet200ResponseBonusStoreBonusStoreOffersInner {
    pub fn new(bonus_offer_id: String, offer: models::StoreV1OffersGet200ResponseOffersInner, discount_percent: f64, discount_costs: std::collections::HashMap<String, f64>, is_seen: bool) -> StoreV2StorefrontPuuidGet200ResponseBonusStoreBonusStoreOffersInner {
        StoreV2StorefrontPuuidGet200ResponseBonusStoreBonusStoreOffersInner {
            bonus_offer_id,
            offer: Box::new(offer),
            discount_percent,
            discount_costs,
            is_seen,
        }
    }
}
