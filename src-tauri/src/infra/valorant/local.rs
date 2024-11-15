use valorant_local::models::_product_session_v1_external_sessions_get_200_response_value::ProductId;

use super::error::APIError;

impl<T: serde::Serialize> From<valorant_local::apis::Error<T>> for APIError {
    fn from(value: valorant_local::apis::Error<T>) -> Self {
        match value {
            valorant_local::apis::Error::Reqwest(e) => {
                APIError::APIError(format!("Reqwest: {:?}", e))
            }
            valorant_local::apis::Error::Io(e) => APIError::APIError(format!("IO: {:?}", e)),
            valorant_local::apis::Error::Serde(e) => APIError::APIError(format!("Serde: {:?}", e)),
            valorant_local::apis::Error::ResponseError(e) => {
                APIError::APIError(format!("status code {}: {}", e.status, e.content))
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct LocalAPI {
    port: String,
    local_password: String,
}

pub struct LocalAPIGetTokenResponse {
    pub token: String,
    pub entitlement: String,
    pub ppuuid: String,
}

pub struct LocalAPIGetRegionResponse {
    pub region: String,
    pub shard: String,
}
impl LocalAPI {
    pub fn new(port: String, local_password: String) -> Self {
        LocalAPI {
            port: port,
            local_password: local_password,
        }
    }

    fn get_config(&self) -> valorant_local::apis::configuration::Configuration {
        let mut config = valorant_local::apis::configuration::Configuration::default();
        config.client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        config.base_path = config.base_path + &self.port;
        config.basic_auth = Some(("riot".to_string(), Some(self.local_password.clone())));
        config
    }

    pub async fn get_token(&self) -> Result<LocalAPIGetTokenResponse, APIError> {
        let res = valorant_local::apis::default_api::entitlements_v1_token_get(&self.get_config())
            .await?;
        Ok(LocalAPIGetTokenResponse {
            token: res.access_token,
            entitlement: res.token,
            ppuuid: res.subject,
        })
    }

    pub async fn get_region(&self) -> Result<LocalAPIGetRegionResponse, APIError> {
        let res = valorant_local::apis::default_api::product_session_v1_external_sessions_get(
            &self.get_config(),
        )
        .await?;
        let parts = res
            .values()
            .find(|v| v.product_id == ProductId::Valorant)
            .map(|v| {
                v.launch_configuration.arguments[4]
                    .split("=")
                    .flat_map(|s| s.split("&"))
                    .collect::<Vec<&str>>()
            })
            .ok_or(APIError::APIError("Failed to get region".to_string()))?;
        match parts[1] {
            "latam" => Ok(LocalAPIGetRegionResponse {
                region: "latam".to_string(),
                shard: "br".to_string(),
            }),
            "na" => Ok(LocalAPIGetRegionResponse {
                region: "na".to_string(),
                shard: "br".to_string(),
            }),
            _ => Ok(LocalAPIGetRegionResponse {
                region: parts[1].to_string(),
                shard: parts[1].to_string(),
            }),
        }
    }
}
