use crate::models::friend_model::FriendRequestType;
use shared::models::friend_models::{FriendRequestRowDto, FriendRowDto};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::AppError, repositories::friend_repository::*};

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

pub async fn service_accept_friend_request(
    db: &PgPool,
    current_user_id: Uuid,
    request_id: Uuid,
) -> Result<(), AppError> {
    accept_friend_request(db, request_id, current_user_id).await?;
    Ok(())
}

pub async fn service_get_friend_requests(
    db: &PgPool,
    current_user_id: Uuid,
    request_type: FriendRequestType,
) -> Result<Vec<FriendRequestRowDto>, AppError> {
    let friend_requests = match request_type {
        FriendRequestType::Sent => get_sent_friend_requests(db, current_user_id).await?,
        FriendRequestType::Received => get_received_friend_requests(db, current_user_id).await?,
    };
    Ok(friend_requests)
}

pub async fn service_get_friends(
    db: &PgPool,
    current_user_id: Uuid,
) -> Result<Vec<FriendRowDto>, AppError> {
    let results = get_friends(db, current_user_id).await?;
    Ok(results)
}
