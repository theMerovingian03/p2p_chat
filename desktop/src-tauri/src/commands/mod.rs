pub mod auth;
pub mod data_channel;
pub mod webrtc;
pub mod websocket;
use tauri::State;

use crate::app_state::AppState;

#[tauri::command]
pub async fn logout_cleanup(state: State<'_, AppState>) -> Result<(), String> {
    // Logout cleanup order:
    // 1. Disconnect WebSocket first to stop accepting new signaling messages
    //    while WebRTC resources are being destroyed. This prevents new events
    //    from arriving and interfering with cleanup.
    state.websocket_manager.disconnect().await;

    // 2. Clear data channels - idempotent operation
    state.dc_manager.clear().await;

    // 3. Clear WebRTC peer connections - idempotent operation
    //    At this point no new signaling messages should arrive
    state.webrtc_manager.clear().await;

    Ok(())
}
