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
pub struct ChatV4FriendsGet200ResponseFriendsInner {
    #[serde(rename = "activePlatform", deserialize_with = "Option::deserialize")]
    pub active_platform: Option<String>,
    #[serde(rename = "displayGroup")]
    pub display_group: String,
    #[serde(rename = "game_name")]
    pub game_name: String,
    #[serde(rename = "game_tag")]
    pub game_tag: String,
    #[serde(rename = "group")]
    pub group: String,
    /// Milliseconds since epoch
    #[serde(rename = "last_online_ts", deserialize_with = "Option::deserialize")]
    pub last_online_ts: Option<f64>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "note")]
    pub note: String,
    #[serde(rename = "pid")]
    pub pid: String,
    /// Player UUID
    #[serde(rename = "puuid")]
    pub puuid: String,
    #[serde(rename = "region")]
    pub region: String,
}

impl ChatV4FriendsGet200ResponseFriendsInner {
    pub fn new(
        active_platform: Option<String>,
        display_group: String,
        game_name: String,
        game_tag: String,
        group: String,
        last_online_ts: Option<f64>,
        name: String,
        note: String,
        pid: String,
        puuid: String,
        region: String,
    ) -> ChatV4FriendsGet200ResponseFriendsInner {
        ChatV4FriendsGet200ResponseFriendsInner {
            active_platform,
            display_group,
            game_name,
            game_tag,
            group,
            last_online_ts,
            name,
            note,
            pid,
            puuid,
            region,
        }
    }
}
