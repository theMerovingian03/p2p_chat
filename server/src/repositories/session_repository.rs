use crate::auth::refresh_token::*;
use crate::models::session_model::Session;
use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_and_store_session(
    user_id: Uuid,
    db: &PgPool,
    refresh_expiration_hours: &i64,
) -> Result<String, sqlx::Error> {
    let rf = get_refresh_token();
    let refresh_token_db = rf.refresh_token_db;
    let refresh_token_client = rf.refresh_token_client;
    let expires_at = Utc::now() + Duration::hours(*refresh_expiration_hours);

    sqlx::query!(
        r#"
            INSERT into sessions (
                user_id,
                refresh_token_hash,
                expires_at,
                last_used
            ) 
            VALUES ($1, $2, $3, NOW())
        "#,
        user_id,
        refresh_token_db,
        expires_at,
    )
    .execute(db)
    .await?;

    Ok(refresh_token_client)
}

pub async fn search_session(
    refresh_token_hash: &str,
    db: &PgPool,
) -> Result<Option<Session>, sqlx::Error> {
    let result = sqlx::query_as::<_, Session>(
        r#"
            SELECT *
            FROM sessions
            WHERE refresh_token_hash = $1
        "#,
    )
    .bind(refresh_token_hash)
    .fetch_optional(db)
    .await?;
    Ok(result)
}

pub async fn rotate_refresh_token(
    session_id: Uuid,
    db: &PgPool,
    refresh_expiration_hours: &i64,
) -> Result<String, sqlx::Error> {
    let rf = get_refresh_token();
    let refresh_token_db = rf.refresh_token_db;
    let refresh_token_client = rf.refresh_token_client;
    let expires_at = Utc::now() + Duration::hours(*refresh_expiration_hours);

    sqlx::query!(
        r#"
            UPDATE sessions 
            SET refresh_token_hash = $1, expires_at = $2, last_used = NOW()
            WHERE id = $3
        "#,
        refresh_token_db,
        expires_at,
        session_id
    )
    .execute(db)
    .await?;

    Ok(refresh_token_client)
}
