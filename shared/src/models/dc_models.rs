use serde::Serialize;
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum DataChannelAppEvent {
    PeerConnected { peer_id: Uuid },
    PeerDisconnected { peer_id: Uuid },
    MessageReceived { peer_id: Uuid, message: Vec<u8> },
    // MessageReceiveError { peer_id: Uuid, error: String },
}
