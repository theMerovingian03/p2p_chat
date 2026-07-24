use crate::auth::jwt::*;
use crate::auth::password::*;
use crate::auth::refresh_token::decode_refresh_token;
use crate::config::Config;
use crate::repositories::auth_repository::*;
use crate::repositories::session_repository::*;
use chrono::Utc;
use names::{Generator, Name};
use rand::{RngExt, distr::Alphanumeric};
use shared::models::auth_models::RefreshSessionRequest;
use shared::models::auth_models::RefreshSessionResponse;
use shared::models::auth_models::{AuthResponse, LoginRequest, RegisterRequest};
use sqlx::PgPool;

use tracing::debug;

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
            let refresh_token =
                create_and_store_session(user.id, db, &config.refresh_expiration_hours).await?;

            Ok(AuthResponse {
                access_token,
                refresh_token,
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
    let refresh_token =
        create_and_store_session(user.id, db, &config.refresh_expiration_hours).await?;

    Ok(AuthResponse {
        access_token,
        refresh_token,
        user: user.into(),
    })
}

pub async fn service_create_guest_user(
    db: &PgPool,
    config: &Config,
) -> Result<AuthResponse, AppError> {
    // Wrap generation in it's own block, since ThreadRng is !Send, this will drop non-Send objects
    let (random_username, random_email, password_hash) = {
        let mut name_generator = Generator::with_naming(Name::Numbered);
        let random_username = name_generator.next().unwrap();
        let random_email = format!("{}{}", random_username.replace("-", ""), "@p2pchat.com");
        let random_password: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        let password_hash = hash_password(&random_password)?;
        (random_username, random_email, password_hash)
    };

    match create_user(
        db,
        &random_email,
        &random_username,
        &password_hash,
        &random_username,
        Some(true),
    )
    .await
    {
        Ok(user) => {
            let access_token =
                create_access_token(user.id, &config.jwt_secret, &config.jwt_expiration_hours)?;
            let refresh_token =
                create_and_store_session(user.id, db, &config.refresh_expiration_hours).await?;
            Ok(AuthResponse {
                access_token,
                refresh_token,
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

pub async fn service_refresh_session(
    request: RefreshSessionRequest,
    config: &Config,
    db: &PgPool,
) -> Result<RefreshSessionResponse, AppError> {
    debug!("Decoding refresh token");
    let refresh_token_hash = decode_refresh_token(&request.refresh_token)?;
    debug!("Decoded refresh token successfully!");
    let session = search_session(&refresh_token_hash, db)
        .await?
        .ok_or(AppError::Unauthorized)?;
    debug!("Found session!");
    if session.expires_at < Utc::now() {
        debug!("Session has expired!");
        return Err(AppError::Unauthorized);
    }

    let access_token = create_access_token(
        session.user_id,
        &config.jwt_secret,
        &config.jwt_expiration_hours,
    )?;

    debug!("generating new refresh token");
    let refresh_token =
        rotate_refresh_token(session.id, db, &config.refresh_expiration_hours).await?;
    debug!("Refresh token generated successfully!");
    Ok(RefreshSessionResponse {
        access_token,
        refresh_token,
    })
}
