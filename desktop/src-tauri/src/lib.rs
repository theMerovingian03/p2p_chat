// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// mod token_store;
mod commands;
use commands::auth::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            save_refresh_token,
            load_refresh_token,
            delete_refresh_token
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
