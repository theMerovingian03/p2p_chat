// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// mod token_store;
mod app_state;
mod commands;
mod utilities;
mod webrtc;
mod websocket;

use crate::app_state::AppState;
use crate::utilities::{
    dc_events::{process_events, DcEvent},
    signalizer::Signaling,
};
use crate::webrtc::manager::WebRtcManager;
use commands::auth::*;
use commands::webrtc::*;
use commands::websocket::*;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing_subscriber::EnvFilter;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use websocket::manager::WebSocketManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(
            "desktop_lib=debug, server=debug,tokio_tungstenite=off",
        ))
        .init();

    let (event_tx, event_rx) = mpsc::channel::<DcEvent>(100);

    // Create WebSocket manager
    let ws_manager = Arc::new(WebSocketManager::new());
    // Create WebRTC Manager
    let webrtc_manager = Arc::new(WebRtcManager::new(
        // Anything that implements Singaling Trait
        // Create Arc pointer for WS manager
        Arc::clone(&ws_manager) as Arc<dyn Signaling>,
        event_tx,
    ));

    tokio::spawn(process_events(event_rx));

    let app_state = AppState {
        websocket_manager: Arc::clone(&ws_manager),
        webrtc_manager: Arc::clone(&webrtc_manager),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state.clone())
        .setup(move |app| {
            // Set the app handle on the WebSocket manager so it can emit events
            ws_manager.set_app_handle(app.handle().clone());
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
            close_peer_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
