use crate::app_state::AppState;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
// TODO: Invoke this for "End Chat"
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
