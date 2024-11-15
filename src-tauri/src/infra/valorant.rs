use std::io;

use glz::GlzAPI;
use local::LocalAPI;
use other::OtherAPI;
use pd::PdAPI;
use shared::SharedAPI;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tauri::async_runtime::block_on;

mod error;
mod glz;
mod local;
mod other;
mod pd;
mod shared;
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
    #[error("Failed to get Pd API")]
    FailedToGetPdAPI,
    #[error("API Error: {0}")]
    APIError(#[from] error::APIError),
    #[error("Failed read lockfile")]
    FailedReadLockfile,
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

#[async_trait::async_trait]
pub trait ValorantAPI: shaku::Interface {
    async fn get_game_state(&self) -> Result<(), ValorantAPIError>;
}

#[derive(shaku::Component)]
#[shaku(interface = ValorantAPI)]
pub struct ValorantAPIImpl {
    #[shaku(default)]
    initialized: Option<Result<(), ValorantAPIError>>,
    #[shaku(default)]
    uni_state: UniState,
    #[shaku(default=Err(ValorantAPIError::NotInitialized("glz".to_string())))]
    glz: Result<GlzAPI, ValorantAPIError>,
    #[shaku(default=Err(ValorantAPIError::NotInitialized("Local".to_string())))]
    local: Result<LocalAPI, ValorantAPIError>,
    #[shaku(default=Err(ValorantAPIError::NotInitialized("Other".to_string())))]
    other: Result<OtherAPI, ValorantAPIError>,
    #[shaku(default=Err(ValorantAPIError::NotInitialized("Pd".to_string())))]
    pd: Result<PdAPI, ValorantAPIError>,
    #[shaku(default=Err(ValorantAPIError::NotInitialized("Shared".to_string())))]
    shared: Result<SharedAPI, ValorantAPIError>,
}

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
impl ValorantAPI for ValorantAPIImpl {
    async fn get_game_state(&self) -> Result<(), ValorantAPIError> {
        match self.get_pd()?.get_token().await {
            Ok(_) => Ok(()),
            Err(e) => Err(ValorantAPIError::APIError(e)),
        }
    }
}

impl ValorantAPIImpl {
    pub fn build() -> Self {
        let mut api = ValorantAPIImpl {
            initialized: None,
            uni_state: UniState::default(),
            glz: Err(ValorantAPIError::NotInitialized("Glz".to_string())),
            local: Err(ValorantAPIError::NotInitialized("Local".to_string())),
            other: Err(ValorantAPIError::NotInitialized("Other".to_string())),
            pd: Err(ValorantAPIError::NotInitialized("Pd".to_string())),
            shared: Err(ValorantAPIError::NotInitialized("Shared".to_string())),
        };
        block_on(api.init());

        api
    }

    async fn init(&mut self) {
        self.init_other();
        match self.load_lockfile() {
            Err(e) => {
                self.initialized = Some(Err(e));
                return;
            }
            Ok(_) => {}
        }
        self.init_local();
        let results = vec![
            self.get_token().await,
            self.get_region().await,
            self.get_version().await,
        ];
        self.init_glz();
        self.init_pd();
        self.init_shared();
        self.initialized = results.into_iter().find(|r| r.is_err());
    }

    fn init_glz(&mut self) {
        match (
            self.uni_state.region.clone(),
            self.uni_state.shard.clone(),
            self.uni_state.entitlement.clone(),
            self.uni_state.version.clone(),
            self.uni_state.platform.clone(),
        ) {
            (Some(region), Some(shard), Some(entitlement), Some(version), Some(platform)) => {
                self.glz = Ok(GlzAPI::new(region, shard, entitlement, version, platform))
            }
            _ => self.glz = Err(ValorantAPIError::NotInitialized("Glz".to_string())),
        }
    }
    fn init_local(&mut self) {
        match (
            self.uni_state.port.clone(),
            self.uni_state.local_password.clone(),
        ) {
            (Some(port), Some(local_password)) => {
                self.local = Ok(LocalAPI::new(port, local_password))
            }
            _ => self.local = Err(ValorantAPIError::NotInitialized("Local".to_string())),
        }
    }
    fn init_other(&mut self) {
        self.other = Ok(OtherAPI::new())
    }
    fn init_pd(&mut self) {
        match (
            self.uni_state.region.clone(),
            self.uni_state.ppuuid.clone(),
            self.uni_state.entitlement.clone(),
            self.uni_state.version.clone(),
            self.uni_state.platform.clone(),
        ) {
            (Some(region), Some(puuid), Some(entitlement), Some(version), Some(platform)) => {
                self.pd = Ok(PdAPI::new(region, puuid, entitlement, version, platform))
            }
            _ => self.pd = Err(ValorantAPIError::NotInitialized("Pd".to_string())),
        }
    }
    fn init_shared(&mut self) {
        match (
            self.uni_state.shard.clone(),
            self.uni_state.entitlement.clone(),
            self.uni_state.version.clone(),
            self.uni_state.platform.clone(),
        ) {
            (Some(shard), Some(entitlement), Some(version), Some(platform)) => {
                self.shared = Ok(SharedAPI::new(shard, entitlement, version, platform))
            }
            _ => self.shared = Err(ValorantAPIError::NotInitialized("Shared".to_string())),
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
    fn get_pd(&self) -> Result<&PdAPI, ValorantAPIError> {
        self.pd.as_ref().or(Err(ValorantAPIError::FailedToGetPdAPI))
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
