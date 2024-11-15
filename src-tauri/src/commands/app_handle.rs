#[cfg_attr(test, mockall::automock)]
pub trait CommandAppHandle {
    fn get_app(&self) -> Result<String, String>;
}

pub struct CommandAppHandleImpl {
    app: tauri::AppHandle,
}

impl CommandAppHandleImpl {
    pub fn new(app: tauri::AppHandle) -> Self {
        Self { app }
    }
}
impl CommandAppHandle for CommandAppHandleImpl {
    fn get_app(&self) -> Result<String, String> {
        Ok("Hello from Rust!".to_string())
    }
}
