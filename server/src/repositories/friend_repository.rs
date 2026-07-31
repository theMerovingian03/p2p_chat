use crate::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_friend_request(
    db: &PgPool,
    sender_user_id: Uuid,
    receiver_user_id: Uuid,
) -> Result<(), AppError> {
    match sqlx::query!(
        r#"
        INSERT INTO friend_requests (sender_id, receiver_id)
        VALUES ($1, $2)
    "#,
        sender_user_id,
        receiver_user_id,
    )
    .execute(db)
    .await
    {
        Ok(_) => Ok(()),

        Err(sqlx::Error::Database(err)) if err.constraint() == Some("unique_friend_pair") => Err(
            AppError::BadRequest("A friend request already exists between these users.".into()),
        ),

        Err(e) => Err(e.into()),
    }
}
