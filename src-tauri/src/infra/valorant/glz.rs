use super::error::APIError;

impl<T: serde::Serialize> From<valorant_glz::apis::Error<T>> for APIError {
    fn from(value: valorant_glz::apis::Error<T>) -> Self {
        match value {
            valorant_glz::apis::Error::Reqwest(e) => {
                APIError::APIError(format!("Reqwest: {:?}", e))
            }
            valorant_glz::apis::Error::Io(e) => APIError::APIError(format!("IO: {:?}", e)),
            valorant_glz::apis::Error::Serde(e) => APIError::APIError(format!("Serde: {:?}", e)),
            valorant_glz::apis::Error::ResponseError(e) => {
                APIError::APIError(format!("status code {}: {}", e.status, e.content))
            }
        }
    }
}
#[derive(Debug, Default)]
pub struct GlzAPI {
    region: String,
    shard: String,
    entitlement: String,
    version: String,
    platform: String,
}
impl GlzAPI {
    pub fn new(
        region: String,
        shard: String,
        entitlement: String,
        version: String,
        platform: String,
    ) -> Self {
        GlzAPI {
            region: region,
            shard: shard,
            entitlement: entitlement,
            version,
            platform: platform,
        }
    }

    fn get_config(&self) -> valorant_glz::apis::configuration::Configuration {
        let mut config = valorant_glz::apis::configuration::Configuration::default();
        config.client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        config.base_path = format!("https://glz-{}-1.{}.a.pvp.net", self.region, self.shard);
        config
    }
}
