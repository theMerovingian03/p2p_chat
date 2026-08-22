use crate::webrtc::manager::WebRtcManager;
use std::sync::Arc;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn close_peer_connection(
    peer_id: String,
    webrtc_manager: State<'_, Arc<WebRtcManager>>,
) -> Result<(), String> {
    let peer_id = Uuid::parse_str(&peer_id).map_err(|e| e.to_string())?;

    webrtc_manager.cleanup_peer_connection(peer_id).await?;

    Ok(())
}
