pub mod auth;
pub mod data_channel;
pub mod webrtc;
pub mod websocket;
use tauri::State;

use crate::app_state::AppState;

#[tauri::command]
pub async fn logout_cleanup(state: State<'_, AppState>) -> Result<(), String> {
    state.dc_manager.clear().await;
    state.webrtc_manager.clear().await;
    state.websocket_manager.disconnect().await;
    Ok(())
}
