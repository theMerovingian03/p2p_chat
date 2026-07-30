use axum::{
    Extension, Json,
    extract::{Query, State},
    http::StatusCode,
};
use shared::models::{
    auth_models::UserDto,
    user_models::{UserSearchModel, UserSearchRequestModel},
};
use uuid::Uuid;

use crate::{
    errors::AppError,
    services::user_service::{service_me, service_search_user},
    state::AppState,
};

pub async fn me(
    Extension(user_id): Extension<Uuid>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<UserDto>), AppError> {
    let user = service_me(&state.db_pool, &user_id).await?;
    Ok((StatusCode::OK, Json(user)))
}

pub async fn search_user(
    Extension(user_id): Extension<Uuid>,
    State(state): State<AppState>,
    Query(params): Query<UserSearchRequestModel>,
) -> Result<(StatusCode, Json<Vec<UserSearchModel>>), AppError> {
    let searches = service_search_user(&state.db_pool, user_id, params).await?;
    Ok((StatusCode::OK, Json(searches)))
}
