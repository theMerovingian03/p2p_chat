use crate::app_state::AppState;
use shared::models::websocket_models::ClientEvent;
use std::sync::Arc;
use tauri::State;
use uuid::Uuid;

/// Initialize WebSocket connection
#[tauri::command]
pub async fn connect_websocket(
    ws_url: String,
    ws_token: String,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    tracing::info!("Connecting to WebSocket: {}", ws_url);
    app_state
        .websocket_manager
        .connect(ws_url, ws_token, Arc::clone(&app_state.webrtc_manager))
        .await;
    Ok(())
}

/// Disconnect from WebSocket
#[tauri::command]
pub async fn disconnect_websocket(app_state: State<'_, AppState>) -> Result<(), String> {
    tracing::info!("Disconnecting from WebSocket");
    app_state.websocket_manager.disconnect().await;
    Ok(())
}

/// Get current WebSocket connection status
#[tauri::command]
pub fn get_websocket_status(app_state: State<'_, AppState>) -> String {
    app_state.websocket_manager.status().as_str().to_string()
}

/// Send a chat request through WebSocket
#[tauri::command]
pub async fn send_chat_request(to: String, app_state: State<'_, AppState>) -> Result<(), String> {
    let to_uuid = Uuid::parse_str(&to).map_err(|e| e.to_string())?;

    let event = ClientEvent::ChatRequestSend { to: to_uuid };
    app_state.websocket_manager.send_event(event).await?;

    Ok(())
}

/// Accept a chat request through WebSocket
#[tauri::command]
pub async fn accept_chat_request(
    from: String,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    let from_uuid = Uuid::parse_str(&from).map_err(|e| e.to_string())?;

    let event = ClientEvent::ChatRequestAccept { from: from_uuid };
    app_state.websocket_manager.send_event(event).await?;

    Ok(())
}
