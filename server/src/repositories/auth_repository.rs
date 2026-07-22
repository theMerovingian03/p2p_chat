use crate::models::user_model::User;
use chrono::{Duration, Utc};
use sqlx::PgPool;

pub async fn create_user(
    db: &PgPool,
    email: &str,
    username: &str,
    password_hash: &str,
    display_name: &str,
    is_guest: Option<bool>,
) -> Result<User, sqlx::Error> {
    let guest = is_guest.unwrap_or(false);
    let expires_at = if guest {
        Some(Utc::now() + Duration::hours(24))
    } else {
        None
    };
    sqlx::query_as::<_, User>(
        "
        INSERT INTO users
    (username, email, password_hash, display_name, is_guest, expires_at)
    VALUES ($1, $2, $3, $4, $5, $6)
    RETURNING *
        ",
    )
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .bind(display_name)
    .bind(guest)
    .bind(expires_at)
    .fetch_one(db)
    .await
}

pub async fn get_user_by_identifier(
    db: &PgPool,
    identifier: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(r#"SELECT * from users WHERE email = $1 OR username = $1"#)
        .bind(identifier)
        .fetch_optional(db)
        .await
}
