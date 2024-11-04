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
pub struct ContentServiceV3ContentGet200ResponseSeasonsInner {
    /// UUID
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub r#type: Type,
    /// Date in ISO 8601 format
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// Date in ISO 8601 format
    #[serde(rename = "EndTime")]
    pub end_time: String,
    #[serde(rename = "IsActive")]
    pub is_active: bool,
}

impl ContentServiceV3ContentGet200ResponseSeasonsInner {
    pub fn new(id: String, name: String, r#type: Type, start_time: String, end_time: String, is_active: bool) -> ContentServiceV3ContentGet200ResponseSeasonsInner {
        ContentServiceV3ContentGet200ResponseSeasonsInner {
            id,
            name,
            r#type,
            start_time,
            end_time,
            is_active,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "episode")]
    Episode,
    #[serde(rename = "act")]
    Act,
}

impl Default for Type {
    fn default() -> Type {
        Self::Episode
    }
}
