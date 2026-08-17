use crate::websocket::WebSocketManager;
use shared::models::websocket_models::ClientEvent;
use std::sync::Arc;
use tauri::State;
use uuid::Uuid;

/// Initialize WebSocket connection
#[tauri::command]
pub async fn connect_websocket(
    ws_url: String,
    ws_token: String,
    ws_manager: State<'_, Arc<WebSocketManager>>,
) -> Result<(), String> {
    tracing::info!("Connecting to WebSocket: {}", ws_url);
    ws_manager.connect(ws_url, ws_token).await;
    Ok(())
}

/// Disconnect from WebSocket
#[tauri::command]
pub async fn disconnect_websocket(
    ws_manager: State<'_, Arc<WebSocketManager>>,
) -> Result<(), String> {
    tracing::info!("Disconnecting from WebSocket");
    ws_manager.disconnect().await;
    Ok(())
}

/// Get current WebSocket connection status
#[tauri::command]
pub fn get_websocket_status(ws_manager: State<'_, Arc<WebSocketManager>>) -> String {
    ws_manager.status().as_str().to_string()
}

/// Send a chat request through WebSocket
#[tauri::command]
pub async fn send_chat_request(
    to: String,
    ws_manager: State<'_, Arc<WebSocketManager>>,
) -> Result<(), String> {
    let to_uuid = Uuid::parse_str(&to).map_err(|e| e.to_string())?;

    let event = ClientEvent::ChatRequestSend { to: to_uuid };
    ws_manager.send_event(event).await?;

    Ok(())
}

/// Accept a chat request through WebSocket
#[tauri::command]
pub async fn accept_chat_request(
    from: String,
    ws_manager: State<'_, Arc<WebSocketManager>>,
) -> Result<(), String> {
    let from_uuid = Uuid::parse_str(&from).map_err(|e| e.to_string())?;

    let event = ClientEvent::ChatRequestAccept { from: from_uuid };
    ws_manager.send_event(event).await?;

    Ok(())
}

/// Send WebRTC offer through WebSocket
#[tauri::command]
pub async fn send_webrtc_offer(
    to: String,
    sdp: String,
    ws_manager: State<'_, Arc<WebSocketManager>>,
) -> Result<(), String> {
    let to_uuid = Uuid::parse_str(&to).map_err(|e| e.to_string())?;

    let event = ClientEvent::WebRtcOffer { to: to_uuid, sdp };
    ws_manager.send_event(event).await?;

    Ok(())
}

/// Send WebRTC answer through WebSocket
#[tauri::command]
pub async fn send_webrtc_answer(
    to: String,
    sdp: String,
    ws_manager: State<'_, Arc<WebSocketManager>>,
) -> Result<(), String> {
    let to_uuid = Uuid::parse_str(&to).map_err(|e| e.to_string())?;

    let event = ClientEvent::WebRtcAnswer { to: to_uuid, sdp };
    ws_manager.send_event(event).await?;

    Ok(())
}

/// Send ICE candidate through WebSocket
#[tauri::command]
pub async fn send_ice_candidate(
    to: String,
    candidate: String,
    ws_manager: State<'_, Arc<WebSocketManager>>,
) -> Result<(), String> {
    let to_uuid = Uuid::parse_str(&to).map_err(|e| e.to_string())?;

    let event = ClientEvent::IceCandidate {
        to: to_uuid,
        candidate,
    };
    ws_manager.send_event(event).await?;

    Ok(())
}
