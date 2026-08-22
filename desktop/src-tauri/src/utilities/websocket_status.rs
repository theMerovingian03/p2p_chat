/// WebSocket connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebSocketStatus {
    Disconnected,
    Connecting,
    Connected,
}

impl WebSocketStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            WebSocketStatus::Disconnected => "disconnected",
            WebSocketStatus::Connecting => "connecting",
            WebSocketStatus::Connected => "connected",
        }
    }
}
