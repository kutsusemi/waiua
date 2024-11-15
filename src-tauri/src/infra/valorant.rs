use std::io;

use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tauri::async_runtime::block_on;
use valorant_local::models::_product_session_v1_external_sessions_get_200_response_value::ProductId;

mod third_party;

#[derive(Debug)]
pub struct UniState {
    port: Option<String>,
    local_password: Option<String>,
    token: Option<String>,
    entitlement: Option<String>,
    ppuuid: Option<String>,
    platform: Option<String>,
    version: Option<String>,
    region: Option<String>,
    shard: Option<String>,
}

impl Default for UniState {
    fn default() -> Self {
        Self {
            port: Default::default(),
            local_password: Default::default(),
            token: Default::default(),
            entitlement: Default::default(),
            ppuuid: Default::default(),
            platform: Some("ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9".to_string()),
            version: Default::default(),
            region: Default::default(),
            shard: Default::default(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ValorantAPIError {
    #[error("{0} API is not initialized")]
    NotInitialized(String),
    #[error("Failed to get Local API")]
    FailedToGetLocalAPI,
    #[error("Local Error: {0}")]
    LocalError(#[from] LocalAPIError),
    #[error("Failed read lockfile")]
    FailedReadLockfile,
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

pub trait ValorantAPI: shaku::Interface {}

#[derive(shaku::Component)]
#[shaku(interface = ValorantAPI)]
pub struct ValorantAPIImpl {
    #[shaku(default)]
    initialized: Option<Result<(), ValorantAPIError>>,
    #[shaku(default)]
    uni_state: UniState,
    #[shaku(default=Err(ValorantAPIError::NotInitialized("Local".to_string())))]
    local: Result<LocalAPI, ValorantAPIError>,
}

impl ValorantAPIImpl {
    pub fn build() -> Self {
        let mut api = ValorantAPIImpl {
            initialized: None,
            uni_state: UniState::default(),
            local: Err(ValorantAPIError::NotInitialized("Local".to_string())),
        };
        block_on(api.init());

        api
    }

    async fn init(&mut self) {
        self.init_local();
        let results = vec![
            self.get_token().await,
            self.get_region().await,
            self.get_version().await,
        ];
        self.initialized = results.into_iter().find(|r| r.is_err());
    }

    fn init_local(&mut self) {
        match self.load_lockfile().and(
            self.uni_state
                .port
                .clone()
                .and_then(|port| {
                    self.uni_state
                        .local_password
                        .clone()
                        .map(|password| (port, password))
                })
                .ok_or(ValorantAPIError::FailedReadLockfile),
        ) {
            Ok((port, local_password)) => self.local = Ok(LocalAPI::new(port, local_password)),
            Err(e) => {
                self.local = Err(e);
                return;
            }
        }
    }

    fn load_lockfile(&mut self) -> Result<(), ValorantAPIError> {
        // fileの場所→%LocalAppData%\Riot Games\Riot Client\Config\lockfile
        let lockfile_path = format!(
            "{}\\Riot Games\\Riot Client\\Config\\lockfile",
            std::env::var("LocalAppData").unwrap()
        );

        let file = File::open(lockfile_path).map_err(ValorantAPIError::IOError)?;
        let line = BufReader::new(file)
            .lines()
            .next()
            .ok_or(ValorantAPIError::FailedReadLockfile)?
            .map_err(ValorantAPIError::IOError)?;
        // `name:pid:port:password:protocol`の形式
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 5 {
            self.uni_state.port = Some(parts[2].to_string());
            self.uni_state.local_password = Some(parts[3].to_string());
            Ok(())
        } else {
            return Err(ValorantAPIError::FailedReadLockfile);
        }
    }
    fn get_local(&self) -> Result<&LocalAPI, ValorantAPIError> {
        self.local
            .as_ref()
            .or(Err(ValorantAPIError::FailedToGetLocalAPI))
    }
    async fn get_token(&mut self) -> Result<(), ValorantAPIError> {
        let res = self.get_local()?.get_token().await?;
        self.uni_state.token = Some(res.token);
        self.uni_state.entitlement = Some(res.entitlement);
        self.uni_state.ppuuid = Some(res.ppuuid);
        Ok(())
    }
    async fn get_region(&mut self) -> Result<(), ValorantAPIError> {
        let res = self.get_local()?.get_region().await?;
        self.uni_state.region = Some(res.region);
        self.uni_state.shard = Some(res.shard);
        Ok(())
    }
    // TODO: ファイルに保存してあげる
    async fn get_version(&mut self) -> Result<(), ValorantAPIError> {
        let res = third_party::version().await?;
        self.uni_state.version = Some(res.data.version);
        Ok(())
    }
}

impl ValorantAPI for ValorantAPIImpl {}

#[derive(thiserror::Error, Debug)]
pub enum LocalAPIError {
    #[error("API error: {0}")]
    APIError(String),
}

impl<T: Serialize> From<valorant_local::apis::Error<T>> for LocalAPIError {
    fn from(value: valorant_local::apis::Error<T>) -> Self {
        match value {
            valorant_local::apis::Error::Reqwest(e) => {
                LocalAPIError::APIError(format!("Reqwest: {:?}", e))
            }
            valorant_local::apis::Error::Io(e) => LocalAPIError::APIError(format!("IO: {:?}", e)),
            valorant_local::apis::Error::Serde(e) => {
                LocalAPIError::APIError(format!("Serde: {:?}", e))
            }
            valorant_local::apis::Error::ResponseError(e) => {
                LocalAPIError::APIError(format!("status code {}: {}", e.status, e.content))
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct LocalAPI {
    port: String,
    local_password: String,
}

struct LocalAPIGetTokenResponse {
    pub token: String,
    pub entitlement: String,
    pub ppuuid: String,
}

struct LocalAPIGetRegionResponse {
    pub region: String,
    pub shard: String,
}
impl LocalAPI {
    fn new(port: String, local_password: String) -> Self {
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

    async fn get_token(&self) -> Result<LocalAPIGetTokenResponse, LocalAPIError> {
        let res = valorant_local::apis::default_api::entitlements_v1_token_get(&self.get_config())
            .await?;
        Ok(LocalAPIGetTokenResponse {
            token: res.access_token,
            entitlement: res.token,
            ppuuid: res.subject,
        })
    }

    async fn get_region(&self) -> Result<LocalAPIGetRegionResponse, LocalAPIError> {
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
            .ok_or(LocalAPIError::APIError("Failed to get region".to_string()))?;
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
