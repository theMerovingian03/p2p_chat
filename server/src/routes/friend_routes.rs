use axum::{Extension, Json, extract::State, http::StatusCode};
use uuid::Uuid;

use crate::models::friend_model::FriendRequestType;
use crate::services::friend_service::*;
use crate::{errors::AppError, state::AppState};
use shared::models::friend_models::{
    AcceptReqRequest, CreateFriendReqRequest, DeleteReqRequest, FriendRequestRowDto, FriendRowDto,
};

#[axum::debug_handler]
pub async fn create_friend_request(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Json(request): Json<CreateFriendReqRequest>,
) -> Result<StatusCode, AppError> {
    service_create_friend_request(&state.db_pool, user_id, request.receiver_id).await?;
    Ok(StatusCode::CREATED)
}

#[axum::debug_handler]
pub async fn accept_friend_request(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Json(request): Json<AcceptReqRequest>,
) -> Result<StatusCode, AppError> {
    service_accept_friend_request(&state.db_pool, user_id, request.request_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[axum::debug_handler]
pub async fn delete_friend_request(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Json(request): Json<DeleteReqRequest>,
) -> Result<StatusCode, AppError> {
    service_delete_friend_request(&state.db_pool, user_id, request.request_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[axum::debug_handler]
pub async fn get_sent_friend_requests(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<(StatusCode, Json<Vec<FriendRequestRowDto>>), AppError> {
    let results =
        service_get_friend_requests(&state.db_pool, user_id, FriendRequestType::Sent).await?;
    Ok((StatusCode::OK, Json(results)))
}

#[axum::debug_handler]
pub async fn get_received_friend_requests(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<(StatusCode, Json<Vec<FriendRequestRowDto>>), AppError> {
    let results =
        service_get_friend_requests(&state.db_pool, user_id, FriendRequestType::Received).await?;
    Ok((StatusCode::OK, Json(results)))
}

pub async fn get_friends(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<(StatusCode, Json<Vec<FriendRowDto>>), AppError> {
    let results = service_get_friends(&state.db_pool, user_id).await?;
    Ok((StatusCode::OK, Json(results)))
}
