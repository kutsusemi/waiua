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
pub struct ContentServiceV3ContentGet200Response {
    #[serde(rename = "DisabledIDs")]
    pub disabled_ids: Vec<serde_json::Value>,
    #[serde(rename = "Seasons")]
    pub seasons: Vec<models::ContentServiceV3ContentGet200ResponseSeasonsInner>,
    #[serde(rename = "Events")]
    pub events: Vec<models::ContentServiceV3ContentGet200ResponseEventsInner>,
}

impl ContentServiceV3ContentGet200Response {
    pub fn new(disabled_ids: Vec<serde_json::Value>, seasons: Vec<models::ContentServiceV3ContentGet200ResponseSeasonsInner>, events: Vec<models::ContentServiceV3ContentGet200ResponseEventsInner>) -> ContentServiceV3ContentGet200Response {
        ContentServiceV3ContentGet200Response {
            disabled_ids,
            seasons,
            events,
        }
    }
}

