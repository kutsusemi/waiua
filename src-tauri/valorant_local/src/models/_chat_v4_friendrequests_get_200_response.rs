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
pub struct ChatV4FriendrequestsGet200Response {
    #[serde(rename = "requests")]
    pub requests: Vec<models::ChatV4FriendrequestsGet200ResponseRequestsInner>,
}

impl ChatV4FriendrequestsGet200Response {
    pub fn new(requests: Vec<models::ChatV4FriendrequestsGet200ResponseRequestsInner>) -> ChatV4FriendrequestsGet200Response {
        ChatV4FriendrequestsGet200Response {
            requests,
        }
    }
}

