// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// mod token_store;
mod commands;
mod websocket;

use commands::auth::*;
use commands::websocket::*;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use websocket::WebSocketManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create WebSocket manager
    let ws_manager = Arc::new(WebSocketManager::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ws_manager.clone())
        .setup(move |app| {
            // Set the app handle on the WebSocket manager so it can emit events
            ws_manager.set_app_handle(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            save_refresh_token,
            load_refresh_token,
            delete_refresh_token,
            connect_websocket,
            disconnect_websocket,
            get_websocket_status,
            send_chat_request,
            accept_chat_request,
            send_webrtc_offer,
            send_webrtc_answer,
            send_ice_candidate,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
