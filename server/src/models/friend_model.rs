use uuid::Uuid;

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
pub struct FriendRequestRow {
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
}
