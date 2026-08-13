use crate::errors::AppError;
use crate::services::auth_service::*;
use crate::state::AppState;
use axum::{
    extract::{Extension, Json, State},
    http::StatusCode,
};
use shared::models::auth_models::{
    AuthResponse, LoginRequest, RefreshSessionRequest, RefreshSessionResponse, RegisterRequest,
    WsAuth,
};
use uuid::Uuid;

#[axum::debug_handler]
pub async fn register_user(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let response = service_register(&state.db_pool, &state.config, request).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[axum::debug_handler]
pub async fn login_user(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let response = service_login(&state.db_pool, request, &state.config).await?;
    Ok((StatusCode::OK, Json(response)))
}

#[axum::debug_handler]
pub async fn create_guest_user(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let response = service_create_guest_user(&state.db_pool, &state.config).await?;
    Ok((StatusCode::OK, Json(response)))
}

#[axum::debug_handler]
pub async fn refresh_session(
    State(state): State<AppState>,
    Json(request): Json<RefreshSessionRequest>,
) -> Result<(StatusCode, Json<RefreshSessionResponse>), AppError> {
    let response = service_refresh_session(request, &state.config, &state.db_pool).await?;
    Ok((StatusCode::OK, Json(response)))
}

#[axum::debug_handler]
pub async fn get_ws_token(
    Extension(user_id): Extension<Uuid>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<WsAuth>), AppError> {
    let response = service_get_ws_token(user_id, &state.config)?;
    Ok((StatusCode::OK, Json(response)))
}
