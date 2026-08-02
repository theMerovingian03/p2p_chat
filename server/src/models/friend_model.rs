use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct FriendRequestRowInternal {
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
}

#[derive(sqlx::FromRow)]
pub struct FriendRequestRow {
    pub id: Uuid,
    pub username: String,
    pub created_at: DateTime<Utc>,
}

pub enum FriendRequestType {
    Sent,
    Received,
}
