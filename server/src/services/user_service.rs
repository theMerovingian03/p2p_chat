use crate::repositories::user_repository::*;
use shared::models::auth_models::UserDto;
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
