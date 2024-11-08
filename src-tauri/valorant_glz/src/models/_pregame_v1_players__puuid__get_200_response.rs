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
pub struct PregameV1PlayersPuuidGet200Response {
    /// Player UUID
    #[serde(rename = "Subject")]
    pub subject: String,
    /// Pre-Game Match ID
    #[serde(rename = "MatchID")]
    pub match_id: String,
    #[serde(rename = "Version")]
    pub version: f64,
}

impl PregameV1PlayersPuuidGet200Response {
    pub fn new(
        subject: String,
        match_id: String,
        version: f64,
    ) -> PregameV1PlayersPuuidGet200Response {
        PregameV1PlayersPuuidGet200Response {
            subject,
            match_id,
            version,
        }
    }
}
