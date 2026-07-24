use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Deserialize, Type)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Debug, Deserialize, Type)]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Serialize, Type)]
pub struct UserDto {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Type)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserDto,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct RefreshSessionRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Type)]
pub struct RefreshSessionResponse {
    pub access_token: String,
    pub refresh_token: String,
}
