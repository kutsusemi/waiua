use super::error::APIError;

impl<T: serde::Serialize> From<valorant_shared::apis::Error<T>> for APIError {
    fn from(value: valorant_shared::apis::Error<T>) -> Self {
        match value {
            valorant_shared::apis::Error::Reqwest(e) => {
                APIError::APIError(format!("Reqwest: {:?}", e))
            }
            valorant_shared::apis::Error::Io(e) => APIError::APIError(format!("IO: {:?}", e)),
            valorant_shared::apis::Error::Serde(e) => APIError::APIError(format!("Serde: {:?}", e)),
            valorant_shared::apis::Error::ResponseError(e) => {
                APIError::APIError(format!("status code {}: {}", e.status, e.content))
            }
        }
    }
}
#[derive(Debug, Default)]
pub struct SharedAPI {
    shard: String,
    entitlement: String,
    version: String,
    platform: String,
}
impl SharedAPI {
    pub fn new(shard: String, entitlement: String, version: String, platform: String) -> Self {
        SharedAPI {
            shard: shard,
            entitlement: entitlement,
            version,
            platform: platform,
        }
    }

    fn get_config(&self) -> valorant_shared::apis::configuration::Configuration {
        let mut config = valorant_shared::apis::configuration::Configuration::default();
        config.client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        config.base_path = format!("https://shared.{}.a.pvp.net", self.shard);
        config
    }

    async fn get_token(&self) -> Result<(), APIError> {
        let a = valorant_shared::apis::default_api::content_service_v3_content_get(
            &self.get_config(),
            self.entitlement.as_str(),
            self.version.as_str(),
            self.platform.as_str(),
        )
        .await?;
        Ok(())
    }
}
