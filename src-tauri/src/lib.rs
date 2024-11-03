use tauri::Manager;

mod commands;
mod domain;
mod infra;
mod module;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let container = module::AppModule::builder().build();
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::hello::hello,
            commands::world::world
        ])
        .setup(|app| {
            app.manage(container);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
