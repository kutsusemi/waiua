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
pub struct ChatV6MessagesGet200ResponseMessagesInner {
    #[serde(rename = "body")]
    pub body: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "game_name")]
    pub game_name: String,
    #[serde(rename = "game_tag")]
    pub game_tag: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "mid")]
    pub mid: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "pid")]
    pub pid: String,
    /// Player UUID
    #[serde(rename = "puuid")]
    pub puuid: String,
    #[serde(rename = "read")]
    pub read: bool,
    #[serde(rename = "region")]
    pub region: String,
    /// Time in milliseconds since epoch
    #[serde(rename = "time")]
    pub time: String,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl ChatV6MessagesGet200ResponseMessagesInner {
    pub fn new(
        body: String,
        cid: String,
        game_name: String,
        game_tag: String,
        id: String,
        mid: String,
        name: String,
        pid: String,
        puuid: String,
        read: bool,
        region: String,
        time: String,
        r#type: Type,
    ) -> ChatV6MessagesGet200ResponseMessagesInner {
        ChatV6MessagesGet200ResponseMessagesInner {
            body,
            cid,
            game_name,
            game_tag,
            id,
            mid,
            name,
            pid,
            puuid,
            read,
            region,
            time,
            r#type,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "chat")]
    Chat,
    #[serde(rename = "groupchat")]
    Groupchat,
}

impl Default for Type {
    fn default() -> Type {
        Self::Chat
    }
}
