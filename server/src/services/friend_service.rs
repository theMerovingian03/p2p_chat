use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::AppError, repositories::friend_repository::create_friend_request};

pub async fn service_create_friend_request(
    db: &PgPool,
    sender_id: Uuid,
    receiver_id: Uuid,
) -> Result<(), AppError> {
    if sender_id == receiver_id {
        return Err(AppError::BadRequest(
            "Cannot send request to same ID!".to_string(),
        ));
    }
    create_friend_request(db, sender_id, receiver_id).await?;
    Ok(())
}
