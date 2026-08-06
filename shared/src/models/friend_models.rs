use chrono::{DateTime, Utc};
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

#[derive(Serialize, Deserialize, Type)]
pub struct FriendRequestRowDto {
    pub id: Uuid,
    pub username: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Type)]
pub struct FriendRowDto {
    pub friend_id: Uuid,
    pub username: String,
    pub display_name: String,
}
