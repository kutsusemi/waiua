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
pub struct HelpGet200Response {
    #[serde(rename = "events")]
    pub events: std::collections::HashMap<String, String>,
    #[serde(rename = "functions")]
    pub functions: std::collections::HashMap<String, String>,
    #[serde(rename = "types")]
    pub types: std::collections::HashMap<String, String>,
}

impl HelpGet200Response {
    pub fn new(
        events: std::collections::HashMap<String, String>,
        functions: std::collections::HashMap<String, String>,
        types: std::collections::HashMap<String, String>,
    ) -> HelpGet200Response {
        HelpGet200Response {
            events,
            functions,
            types,
        }
    }
}
