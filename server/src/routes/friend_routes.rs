use axum::{Extension, Json, extract::State, http::StatusCode};
use uuid::Uuid;

use crate::services::friend_service::service_create_friend_request;
use crate::{errors::AppError, state::AppState};
use shared::models::friend_models::CreateFriendReqRequest;

#[axum::debug_handler]
pub async fn create_friend_request(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Json(request): Json<CreateFriendReqRequest>,
) -> Result<StatusCode, AppError> {
    service_create_friend_request(&state.db_pool, user_id, request.receiver_id).await?;
    Ok(StatusCode::OK)
}
