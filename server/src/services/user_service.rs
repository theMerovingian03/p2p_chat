use crate::repositories::user_repository::*;
use axum::extract::Extension;
use shared::models::{
    auth_models::UserDto,
    user_models::{UserSearchModel, UserSearchRequestModel},
};
use sqlx::PgPool;
use tokio::time::{Duration, interval};
use uuid::Uuid;

use crate::errors::AppError;

pub async fn service_me(db: &PgPool, id: &Uuid) -> Result<UserDto, AppError> {
    let user = get_user_by_id(db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found!".into()))?;
    Ok(user.into())
}

pub async fn service_search_user(
    db: &PgPool,
    current_user: Uuid,
    request: UserSearchRequestModel,
) -> Result<Vec<UserSearchModel>, AppError> {
    let searches = search_user(db, current_user, &request.query).await?;
    Ok(searches)
}

pub async fn guest_cleanup_task(db: PgPool) {
    let mut interval = interval(Duration::from_secs(60 * 60));

    loop {
        interval.tick().await;

        match cleanup_guest_accounts(&db).await {
            Ok(count) => {
                tracing::info!(deleted = count, "Guest cleanup completed");
            }
            Err(error) => {
                tracing::error!(?error, "Failed to cleanup guest accounts");
            }
        }
    }
}
