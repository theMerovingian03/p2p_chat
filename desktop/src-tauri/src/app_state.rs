use crate::webrtc::manager::WebRtcManager;
use crate::websocket::manager::WebSocketManager;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub websocket_manager: Arc<WebSocketManager>,
    pub webrtc_manager: Arc<WebRtcManager>,
}
