use shared::models::user_models::UserSearchModel;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user_model::{User, UserSearchRow};

pub async fn get_user_by_id(db: &PgPool, id: &Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(r#"SELECT * from users WHERE id = $1"#)
        .bind(id)
        .fetch_optional(db)
        .await
}

pub async fn cleanup_guest_accounts(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM users
        WHERE is_guest = TRUE
            AND created_at < NOW() - INTERVAL '24 hours'
    "#,
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

pub async fn search_user(
    pool: &PgPool,
    current_user: Uuid,
    query: &str,
) -> Result<Vec<UserSearchModel>, sqlx::Error> {
    let pattern = format!("%{}%", query);
    let rows = sqlx::query_as::<_, UserSearchRow>(
        // TODO: Add email search
        r#"SELECT
            u.id,
            u.username,
            u.email
        FROM users u

        WHERE
            u.id != $1
            AND u.username ILIKE $2

            AND NOT EXISTS (
            
                SELECT 1
                FROM friends f

                WHERE
                    (f.user_id = $1 AND f.friend_id = u.id)
                    OR
                    (f.friend_id = $1 AND f.user_id = u.id)
            )
        ORDER BY u.username
        LIMIT 20"#,
    )
    .bind(current_user)
    .bind(pattern)
    .persistent(false)
    .fetch_all(pool)
    .await?;

    let users = rows
        .into_iter()
        .map(|r| UserSearchModel {
            id: r.id,
            username: r.username,
            email: r.email,
        })
        .collect();

    Ok(users)
}
