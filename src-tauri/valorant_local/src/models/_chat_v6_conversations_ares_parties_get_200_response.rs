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
pub struct ChatV6ConversationsAresPartiesGet200Response {
    #[serde(rename = "conversations")]
    pub conversations: Vec<models::ChatV6ConversationsAresPartiesGet200ResponseConversationsInner>,
}

impl ChatV6ConversationsAresPartiesGet200Response {
    pub fn new(
        conversations: Vec<models::ChatV6ConversationsAresPartiesGet200ResponseConversationsInner>,
    ) -> ChatV6ConversationsAresPartiesGet200Response {
        ChatV6ConversationsAresPartiesGet200Response { conversations }
    }
}
