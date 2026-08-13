use std::vec;

use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone, Type, Serialize)]
#[serde(tag = "type")]
pub enum WsErrorCode {
    UserNotFound,
    UserOffline,
    NotFriends,
    // RequestAlreadyExists,
    Unauthorized,
    InvalidRequest,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
#[serde(tag = "type")]
pub enum ServerEvent {
    ChatRequestIncoming { from: Uuid },
    ChatRequestAccepted { from: Uuid },
    PresenceOnline { id: Uuid },
    PresenceOffline { id: Uuid },
    WebRtcOffer { from: Uuid, sdp: String },
    WebRtcAnswer { from: Uuid, sdp: String },
    IceCandidate { from: Uuid, candidate: String },
    Error { code: WsErrorCode, message: String },

    GenericMessage { message: String },
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type")]
pub enum ClientEvent {
    // RequestFriendPresences { friend_list: Vec<Uuid> },
    ChatRequestSend { to: Uuid },
    ChatRequestAccept { from: Uuid },
    WebRtcOffer { to: Uuid, sdp: String },
    WebRtcAnswer { to: Uuid, sdp: String },
    IceCandidate { to: Uuid, candidate: String },
}
