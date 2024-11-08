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
pub struct NameServiceV2PlayersPut200ResponseInner {
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    /// Player UUID
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "GameName")]
    pub game_name: String,
    #[serde(rename = "TagLine")]
    pub tag_line: String,
}

impl NameServiceV2PlayersPut200ResponseInner {
    pub fn new(
        display_name: String,
        subject: String,
        game_name: String,
        tag_line: String,
    ) -> NameServiceV2PlayersPut200ResponseInner {
        NameServiceV2PlayersPut200ResponseInner {
            display_name,
            subject,
            game_name,
            tag_line,
        }
    }
}
