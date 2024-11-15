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

pub mod tests {
    use super::*;

    pub struct CommandAppHandleMock {}
    impl CommandAppHandleMock {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl CommandAppHandle for CommandAppHandleMock {
        fn get_app(&self) -> Result<String, String> {
            Ok("Hello from Test!".to_string())
        }
    }
}
