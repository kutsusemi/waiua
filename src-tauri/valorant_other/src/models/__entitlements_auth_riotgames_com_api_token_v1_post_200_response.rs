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
pub struct EntitlementsAuthRiotgamesComApiTokenV1Post200Response {
    #[serde(rename = "entitlements_token")]
    pub entitlements_token: String,
}

impl EntitlementsAuthRiotgamesComApiTokenV1Post200Response {
    pub fn new(
        entitlements_token: String,
    ) -> EntitlementsAuthRiotgamesComApiTokenV1Post200Response {
        EntitlementsAuthRiotgamesComApiTokenV1Post200Response { entitlements_token }
    }
}
