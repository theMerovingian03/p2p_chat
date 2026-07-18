use crate::repositories::user_repository::get_user_by_id;
use shared::models::auth_models::UserDto;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;

pub async fn service_me(db: &PgPool, id: &Uuid) -> Result<UserDto, AppError> {
    let user = get_user_by_id(db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found!".into()))?;
    Ok(UserDto {
        id: user.id,
        username: user.username,
        display_name: user.display_name,
        email: user.email,
    })
}
