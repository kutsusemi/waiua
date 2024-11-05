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

/// PersonalizationV2PlayersPuuidPlayerloadoutGet200ResponseGunsInner : Guns and knife. Note that the knife (ID: 2f59173c-4bed-b6c3-2191-dea9b58be9c7) does not have charm data (buddies).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersonalizationV2PlayersPuuidPlayerloadoutGet200ResponseGunsInner {
    /// UUID
    #[serde(rename = "ID")]
    pub id: String,
    /// UUID
    #[serde(rename = "CharmInstanceID", skip_serializing_if = "Option::is_none")]
    pub charm_instance_id: Option<String>,
    /// UUID
    #[serde(rename = "CharmID", skip_serializing_if = "Option::is_none")]
    pub charm_id: Option<String>,
    /// UUID
    #[serde(rename = "CharmLevelID", skip_serializing_if = "Option::is_none")]
    pub charm_level_id: Option<String>,
    /// UUID
    #[serde(rename = "SkinID")]
    pub skin_id: String,
    /// UUID
    #[serde(rename = "SkinLevelID")]
    pub skin_level_id: String,
    /// UUID
    #[serde(rename = "ChromaID")]
    pub chroma_id: String,
    #[serde(rename = "Attachments")]
    pub attachments: Vec<serde_json::Value>,
}

impl PersonalizationV2PlayersPuuidPlayerloadoutGet200ResponseGunsInner {
    /// Guns and knife. Note that the knife (ID: 2f59173c-4bed-b6c3-2191-dea9b58be9c7) does not have charm data (buddies).
    pub fn new(
        id: String,
        skin_id: String,
        skin_level_id: String,
        chroma_id: String,
        attachments: Vec<serde_json::Value>,
    ) -> PersonalizationV2PlayersPuuidPlayerloadoutGet200ResponseGunsInner {
        PersonalizationV2PlayersPuuidPlayerloadoutGet200ResponseGunsInner {
            id,
            charm_instance_id: None,
            charm_id: None,
            charm_level_id: None,
            skin_id,
            skin_level_id,
            chroma_id,
            attachments,
        }
    }
}
