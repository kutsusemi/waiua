pub trait MyAppHandle {
    fn get_app(&self) -> Result<String, String>;
}

pub struct MyAppHandleImpl {
    app: tauri::AppHandle,
}

impl MyAppHandleImpl {
    pub fn new(app: tauri::AppHandle) -> Self {
        Self { app }
    }
}
impl MyAppHandle for MyAppHandleImpl {
    fn get_app(&self) -> Result<String, String> {
        Ok("Hello from Rust!".to_string())
    }
}

pub mod tests {
    use super::*;

    pub struct MyAppandleMock {}
    impl MyAppandleMock {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl MyAppHandle for MyAppandleMock {
        fn get_app(&self) -> Result<String, String> {
            Ok("Hello from Test!".to_string())
        }
    }
}
