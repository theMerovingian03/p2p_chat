use crate::auth::jwt::*;
use crate::auth::password::*;
use crate::config::Config;
use crate::repositories::auth_repository::*;
use shared::models::auth_models::{AuthResponse, LoginRequest, RegisterRequest};
use sqlx::PgPool;

use crate::errors::AppError;

pub async fn service_register(
    db: &PgPool,
    config: &Config,
    request: RegisterRequest,
) -> Result<AuthResponse, AppError> {
    let password_hash = hash_password(&request.password)?;
    match create_user(
        db,
        &request.email,
        &request.username,
        &password_hash,
        &request.display_name,
        None,
    )
    .await
    {
        Ok(user) => {
            let access_token =
                create_access_token(user.id, &config.jwt_secret, &config.jwt_expiration_hours)?;
            Ok(AuthResponse {
                access_token,
                user: user.into(),
            })
        }
        Err(sqlx::Error::Database(db_error)) => match db_error.constraint() {
            Some("users_email_key") => Err(AppError::Conflict("Email already registered".into())),
            Some("users_username_key") => Err(AppError::Conflict("Username already taken".into())),
            _ => Err(AppError::Conflict("Duplicate value".into())),
        },
        Err(e) => Err(e.into()),
    }
}

pub async fn service_login(
    db: &PgPool,
    request: LoginRequest,
    config: &Config,
) -> Result<AuthResponse, AppError> {
    // Fetch user
    let user = match get_user_by_identifier(db, &request.identifier).await? {
        Some(user) => user,
        None => return Err(AppError::Unauthorized),
    };

    if !verify_password(&request.password, &user.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    // Password verified so create access token
    let access_token =
        create_access_token(user.id, &config.jwt_secret, &config.jwt_expiration_hours)?;

    Ok(AuthResponse {
        access_token,
        user: user.into(),
    })
}

// pub async fn create_guest_user(
//     db: &PgPool,
//     config: &Config,
// ) -> Result<AuthResponse, AppError> {

// }
