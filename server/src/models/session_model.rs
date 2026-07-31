use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
}
