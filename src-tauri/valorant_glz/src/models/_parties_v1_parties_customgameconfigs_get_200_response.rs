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
pub struct PartiesV1PartiesCustomgameconfigsGet200Response {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "EnabledMaps")]
    pub enabled_maps: Vec<String>,
    #[serde(rename = "EnabledModes")]
    pub enabled_modes: Vec<String>,
    #[serde(rename = "Queues")]
    pub queues: Vec<models::PartiesV1PartiesCustomgameconfigsGet200ResponseQueuesInner>,
    #[serde(rename = "GamePodPingServiceInfo")]
    pub game_pod_ping_service_info: std::collections::HashMap<
        String,
        models::PartiesV1PartiesCustomgameconfigsGet200ResponseGamePodPingServiceInfoValue,
    >,
}

impl PartiesV1PartiesCustomgameconfigsGet200Response {
    pub fn new(
        enabled: bool,
        enabled_maps: Vec<String>,
        enabled_modes: Vec<String>,
        queues: Vec<models::PartiesV1PartiesCustomgameconfigsGet200ResponseQueuesInner>,
        game_pod_ping_service_info: std::collections::HashMap<
            String,
            models::PartiesV1PartiesCustomgameconfigsGet200ResponseGamePodPingServiceInfoValue,
        >,
    ) -> PartiesV1PartiesCustomgameconfigsGet200Response {
        PartiesV1PartiesCustomgameconfigsGet200Response {
            enabled,
            enabled_maps,
            enabled_modes,
            queues,
            game_pod_ping_service_info,
        }
    }
}
