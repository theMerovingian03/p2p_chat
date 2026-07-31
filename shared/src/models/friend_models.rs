use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Type)]
pub struct CreateFriendReqRequest {
    pub receiver_id: Uuid,
}

#[derive(Serialize, Deserialize, Type)]
pub struct AcceptReqRequest {
    pub request_id: Uuid,
}
