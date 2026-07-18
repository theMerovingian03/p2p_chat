use axum::{Extension, Json, extract::State, http::StatusCode};
use shared::models::auth_models::UserDto;
use uuid::Uuid;

use crate::{errors::AppError, services::user_service::service_me, state::AppState};

pub async fn me(
    Extension(user_id): Extension<Uuid>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<UserDto>), AppError> {
    let user = service_me(&state.db_pool, &user_id).await?;
    Ok((StatusCode::OK, Json(user)))
}
