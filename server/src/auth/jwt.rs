use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::claims_model::{Claims, WsClaims},
};

pub fn create_access_token(
    user_id: Uuid,
    jwt_secret: &str,
    jwt_expiration_hours: &i64,
) -> Result<String, AppError> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id.to_string(),
        iat: now.timestamp(),
        exp: (now + Duration::hours(*jwt_expiration_hours)).timestamp(),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn verify_access_token(token: &str, jwt_secret: &str) -> Result<Claims, AppError> {
    let token_data = decode(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

pub fn create_ws_token(
    user_id: Uuid,
    jwt_secret: &str,
    jwt_expiration_minutes: &i64,
) -> Result<String, AppError> {
    let now = Utc::now();
    let claims = WsClaims {
        sub: user_id.to_string(),
        token_type: "ws_token".to_string(),
        iat: now.timestamp(),
        exp: (now + Duration::minutes(*jwt_expiration_minutes)).timestamp(),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn verify_ws_token(token: &str, jwt_secret: &str) -> Result<WsClaims, AppError> {
    let token_data = decode(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
