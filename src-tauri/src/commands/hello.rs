use super::app_handle::{MyAppHandle, MyAppHandleImpl};
fn hello_exec(app_handle: impl MyAppHandle) -> Result<(), String> {
    print!("{}", app_handle.get_app()?);
    Ok(())
}

#[tauri::command]
pub fn hello(app_handle: tauri::AppHandle) -> Result<(), String> {
    let app_handle = MyAppHandleImpl::new(app_handle);
    hello_exec(app_handle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::app_handle::tests;
    #[test]
    fn should_return_0_with_no_loaded_models() {
        let app_handle = tests::MyAppandleMock::new();
        assert_eq!(hello_exec(app_handle).unwrap(), ());
    }
}
