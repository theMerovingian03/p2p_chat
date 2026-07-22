use crate::models::user_model::User;
use sqlx::PgPool;

pub async fn create_user(
    db: &PgPool,
    email: &str,
    username: &str,
    password_hash: &str,
    display_name: &str,
    is_guest: Option<bool>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "
        INSERT INTO users
    (username, email, password_hash, display_name, is_guest)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING *
        ",
    )
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .bind(display_name)
    .bind(is_guest.unwrap_or(false))
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
