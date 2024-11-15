use super::error::APIError;

impl<T: serde::Serialize> From<valorant_pd::apis::Error<T>> for APIError {
    fn from(value: valorant_pd::apis::Error<T>) -> Self {
        match value {
            valorant_pd::apis::Error::Reqwest(e) => APIError::APIError(format!("Reqwest: {:?}", e)),
            valorant_pd::apis::Error::Io(e) => APIError::APIError(format!("IO: {:?}", e)),
            valorant_pd::apis::Error::Serde(e) => APIError::APIError(format!("Serde: {:?}", e)),
            valorant_pd::apis::Error::ResponseError(e) => {
                APIError::APIError(format!("status code {}: {}", e.status, e.content))
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct PdAPI {
    region: String,
    puuid: String,
    entitlement: String,
    version: String,
    platform: String,
}

impl PdAPI {
    pub fn new(
        region: String,
        puuid: String,
        entitlement: String,
        version: String,
        platform: String,
    ) -> Self {
        PdAPI {
            region: region,
            puuid: puuid,
            entitlement: entitlement,
            version,
            platform: platform,
        }
    }

    fn get_config(&self) -> valorant_pd::apis::configuration::Configuration {
        let mut config = valorant_pd::apis::configuration::Configuration::default();
        config.client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        config.base_path = format!("https://pd.{}.a.pvp.net", self.region);
        config
    }

    pub async fn get_token(&self) -> Result<(), APIError> {
        let a = valorant_pd::apis::default_api::account_xp_v1_players_puuid_get(
            &self.get_config(),
            self.puuid.as_str(),
            self.entitlement.as_str(),
            self.version.as_str(),
            self.platform.as_str(),
        )
        .await?;
        Ok(())
    }
}
