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
pub struct RsoAuthV1AuthorizationUserinfoGet200Response {
    #[serde(rename = "userInfo")]
    pub user_info: String,
}

impl RsoAuthV1AuthorizationUserinfoGet200Response {
    pub fn new(user_info: String) -> RsoAuthV1AuthorizationUserinfoGet200Response {
        RsoAuthV1AuthorizationUserinfoGet200Response {
            user_info,
        }
    }
}
