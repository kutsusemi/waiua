use super::error::APIError;

impl<T: serde::Serialize> From<valorant_other::apis::Error<T>> for APIError {
    fn from(value: valorant_other::apis::Error<T>) -> Self {
        match value {
            valorant_other::apis::Error::Reqwest(e) => {
                APIError::APIError(format!("Reqwest: {:?}", e))
            }
            valorant_other::apis::Error::Io(e) => APIError::APIError(format!("IO: {:?}", e)),
            valorant_other::apis::Error::Serde(e) => APIError::APIError(format!("Serde: {:?}", e)),
            valorant_other::apis::Error::ResponseError(e) => {
                APIError::APIError(format!("status code {}: {}", e.status, e.content))
            }
        }
    }
}
#[derive(Debug, Default)]
pub struct OtherAPI {}
impl OtherAPI {
    pub fn new() -> Self {
        OtherAPI {}
    }

    fn get_config(&self) -> valorant_other::apis::configuration::Configuration {
        let mut config = valorant_other::apis::configuration::Configuration::default();
        config.client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        config.base_path = "https:".into();
        config
    }
}
