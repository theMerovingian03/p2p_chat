use crate::app_state::AppState;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn send_message(
    peer_id: String,
    message: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let peer_id = Uuid::parse_str(&peer_id).map_err(|e| e.to_string())?;

    state
        .webrtc_manager
        .send_message(peer_id, message.into_bytes())
        .await?;
    Ok(())
}

#[tauri::command]
pub async fn close_peer_connection(
    peer_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let peer_id = Uuid::parse_str(&peer_id).map_err(|e| e.to_string())?;

    state
        .webrtc_manager
        .cleanup_peer_connection(peer_id)
        .await?;

    Ok(())
}
