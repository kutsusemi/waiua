/*
 * Valorant API
 *
 * Valorant API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`content_service_v3_content_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentServiceV3ContentGetError {
    UnknownValue(serde_json::Value),
}

/// Get a list of seasons, acts, and events
pub async fn content_service_v3_content_get(
    configuration: &configuration::Configuration,
    x_riot_entitlements_jwt: &str,
    x_riot_client_version: &str,
    x_riot_client_platform: &str,
) -> Result<models::ContentServiceV3ContentGet200Response, Error<ContentServiceV3ContentGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/content_service/v3/content",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header(
        "X-Riot-Entitlements-JWT",
        x_riot_entitlements_jwt.to_string(),
    );
    local_var_req_builder =
        local_var_req_builder.header("X-Riot-ClientVersion", x_riot_client_version.to_string());
    local_var_req_builder =
        local_var_req_builder.header("X-Riot-ClientPlatform", x_riot_client_platform.to_string());
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ContentServiceV3ContentGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
