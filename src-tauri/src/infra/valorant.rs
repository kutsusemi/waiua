use std::io;

use std::fs::File;
use std::io::{BufRead, BufReader};
use tauri::async_runtime::block_on;

#[derive(Default)]
pub struct UniState {
    port: Option<String>,
    local_password: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum ValorantAPIError {
    #[error("{0} API is not initialized")]
    NotInitialized(String),
    #[error("Local Error: {0}")]
    LocalError(#[from] LocalError),
    #[error("Failed read lockfile")]
    FailedReadLockfile,
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
}

pub trait ValorantAPI: shaku::Interface {}

#[derive(shaku::Component)]
#[shaku(interface = ValorantAPI)]
pub struct ValorantAPIImpl {
    #[shaku(default)]
    uni_state: UniState,
    #[shaku(default=Err(ValorantAPIError::NotInitialized("Local".to_string())))]
    local: Result<Local, ValorantAPIError>,
}

impl ValorantAPIImpl {
    pub fn build() -> Self {
        let mut api = ValorantAPIImpl {
            uni_state: UniState::default(),
            local: Err(ValorantAPIError::NotInitialized("Local".to_string())),
        };
        api.init();

        api
    }

    fn init(&mut self) {
        self.init_local();
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
            Ok((port, local_password)) => self.local = Ok(Local::new(port, local_password)),
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
            .map_err(|e| ValorantAPIError::IOError(e))?;
        // `name:pid:port:password:protocol`の形式
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 5 {
            self.uni_state.port = Some(parts[2].to_string());
            self.uni_state.local_password = Some(parts[3].to_string());
        } else {
            return Err(ValorantAPIError::FailedReadLockfile);
        }
        Ok(())
    }
}

impl ValorantAPI for ValorantAPIImpl {}

#[derive(thiserror::Error, Debug)]
pub enum LocalError {}

pub struct Local {
    port: String,
    local_password: String,
}

impl Default for Local {
    fn default() -> Self {
        Local {
            port: "".to_string(),
            local_password: "".to_string(),
        }
    }
}
impl Local {
    fn new(port: String, local_password: String) -> Self {
        Local {
            port: port,
            local_password: local_password,
        }
    }
}
