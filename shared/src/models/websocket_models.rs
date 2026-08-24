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

#[derive(Debug, Deserialize, Clone, Type, Serialize)]
pub struct IceCandidate {
    pub candidate: String,
    pub sdp_mid: Option<String>,
    pub sdp_mline_index: Option<u16>,
    pub username_fragment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
#[serde(tag = "type")]
pub enum ServerEvent {
    ChatRequestIncoming { from: Uuid, username: String },
    ChatRequestAccepted { from: Uuid },
    PresenceOnline { id: Uuid },
    PresenceOffline { id: Uuid },
    WebRtcOffer { from: Uuid, sdp: String },
    WebRtcAnswer { from: Uuid, sdp: String },
    IceCandidate { from: Uuid, candidate: IceCandidate },
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
    IceCandidate { to: Uuid, candidate: IceCandidate },
}
