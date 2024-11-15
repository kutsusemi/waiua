use infra::valorant::{ValorantAPI, ValorantAPIImpl};
use tauri::Manager;

mod commands;
mod domain;
mod infra;
mod module;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_game_state::get_game_state,
        ])
        .setup(|app| {
            let container = module::AppModule::builder()
                .with_component_override::<dyn ValorantAPI>(Box::new(ValorantAPIImpl::build()))
                .build();
            app.manage(container);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
