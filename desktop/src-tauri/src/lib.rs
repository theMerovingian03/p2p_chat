// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod app_state;
mod commands;
mod data_channel;
mod utilities;
mod webrtc;
mod websocket;

use crate::app_state::AppState;
use crate::data_channel::dc_manager::DcManager;
use crate::utilities::{dc_events::DcEvent, signalizer::Signaling};
use crate::webrtc::manager::WebRtcManager;
use commands::auth::*;
use commands::data_channel::*;
use commands::webrtc::*;
use commands::websocket::*;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing_subscriber::EnvFilter;
use websocket::manager::WebSocketManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("desktop_lib=debug, tokio_tungstenite=off"))
        .init();

    let (event_tx, event_rx) = mpsc::channel::<DcEvent>(100);

    // Create WebSocket manager
    let ws_manager = Arc::new(WebSocketManager::new());

    // Create DataChannel Mnanager
    let dc_manager = Arc::new(DcManager::new(event_tx));

    // Create WebRTC Manager
    let webrtc_manager = Arc::new(WebRtcManager::new(
        // Anything that implements Singaling Trait
        // Create Arc pointer for WS manager
        Arc::clone(&ws_manager) as Arc<dyn Signaling>,
        Arc::clone(&dc_manager),
    ));

    let app_state = AppState {
        websocket_manager: Arc::clone(&ws_manager),
        webrtc_manager: Arc::clone(&webrtc_manager),
        dc_manager: Arc::clone(&dc_manager),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state.clone())
        .setup(move |app| {
            // Set the app handle on the WebSocket manager so it can emit events
            ws_manager.set_app_handle(app.handle().clone());
            // Clone manager to...
            let manager = Arc::clone(&dc_manager);
            manager.set_app_handle(app.handle().clone());
            // ...spawn process to handle datachannel event
            tauri::async_runtime::spawn(async move {
                manager.process_events(event_rx).await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Auth
            save_refresh_token,
            load_refresh_token,
            delete_refresh_token,
            // Websocket
            connect_websocket,
            disconnect_websocket,
            get_websocket_status,
            send_chat_request,
            accept_chat_request,
            // WebRTC
            close_peer_connection,
            // Data Channel
            send_message,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
