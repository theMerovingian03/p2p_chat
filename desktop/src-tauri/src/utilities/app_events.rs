use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum AppEvent {
    PeerConnected { peer_id: Uuid },
    PeerDisconnected { peer_id: Uuid },
    MessageReceived { peer_id: Uuid, message: Vec<u8> },
    // MessageReceiveError { peer_id: Uuid, error: String },
}
