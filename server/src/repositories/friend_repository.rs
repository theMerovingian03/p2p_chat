use crate::errors::AppError;
use crate::models::friend_model::{FriendRequestRow, FriendRequestRowInternal, FriendRow};
use shared::models::friend_models::{FriendRequestRowDto, FriendRowDto};
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

pub async fn accept_friend_request(
    db: &PgPool,
    request_id: Uuid,
    current_user_id: Uuid,
) -> Result<(), AppError> {
    // Step 1: Create transaction
    let mut tx = db.begin().await?;
    // Step 2: Find the specific request
    let request = sqlx::query_as::<_, FriendRequestRowInternal>(
        r#"
            SELECT sender_id, receiver_id
            FROM friend_requests
            WHERE id = $1
        "#,
    )
    .bind(request_id)
    .fetch_optional(&mut *tx)
    .await?;

    let friend_request = request.ok_or(AppError::NotFound(
        "Could not find this friend request!".into(),
    ))?;

    if friend_request.receiver_id != current_user_id {
        return Err(AppError::Forbidden);
    }
    // Step 3: Create an entry to record friendship
    sqlx::query!(
        r#"
        INSERT INTO friends (user_id, friend_id)
        VALUES ($1, $2)
        "#,
        current_user_id,
        friend_request.sender_id
    )
    .execute(&mut *tx)
    .await?;
    // Step 4: Delete friend request
    sqlx::query!(
        r#"
        DELETE FROM friend_requests
        WHERE id = $1
        "#,
        request_id
    )
    .execute(&mut *tx)
    .await?;
    // Step 5: Commit transaction
    tx.commit().await?;

    Ok(())
}

pub async fn delete_friend_request(
    db: &PgPool,
    request_id: Uuid,
    current_user_id: Uuid,
) -> Result<(), AppError> {
    let request = sqlx::query_as::<_, FriendRequestRowInternal>(
        r#"
            SELECT sender_id, receiver_id
            FROM friend_requests
            WHERE id = $1
        "#,
    )
    .bind(request_id)
    .fetch_optional(db)
    .await?;

    let request = request.ok_or(AppError::NotFound(
        "Could not find this friend request!".into(),
    ))?;

    if request.sender_id != current_user_id && request.receiver_id != current_user_id {
        return Err(AppError::Forbidden);
    }

    sqlx::query!(
        r#"
        DELETE FROM friend_requests
        WHERE id = $1
        "#,
        request_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn get_sent_friend_requests(
    db: &PgPool,
    current_user_id: Uuid,
) -> Result<Vec<FriendRequestRowDto>, sqlx::Error> {
    let results = sqlx::query_as::<_, FriendRequestRow>(
        r#"
            SELECT fr.id,
            u.username,
            fr.created_at

            FROM friend_requests fr
            JOIN users u
            
            ON u.id = fr.receiver_id
            WHERE fr.sender_id = $1
            ORDER BY fr.created_at DESC
        "#,
    )
    .bind(current_user_id)
    .fetch_all(db)
    .await?;

    let friend_requests = results
        .into_iter()
        .map(|req| FriendRequestRowDto {
            id: req.id,
            username: req.username,
            created_at: req.created_at,
        })
        .collect();

    Ok(friend_requests)
}

pub async fn get_received_friend_requests(
    db: &PgPool,
    current_user_id: Uuid,
) -> Result<Vec<FriendRequestRowDto>, sqlx::Error> {
    let results = sqlx::query_as::<_, FriendRequestRow>(
        r#"
            SELECT fr.id,
            u.username,
            fr.created_at

            FROM friend_requests fr
            JOIN users u
            
            ON u.id = fr.sender_id
            WHERE fr.receiver_id = $1
            ORDER BY fr.created_at DESC
        "#,
    )
    .bind(current_user_id)
    .fetch_all(db)
    .await?;

    let friend_requests = results
        .into_iter()
        .map(|req| FriendRequestRowDto {
            id: req.id,
            username: req.username,
            created_at: req.created_at,
        })
        .collect();

    Ok(friend_requests)
}

// TODO: Add pagination
pub async fn get_friends(
    db: &PgPool,
    current_user_id: Uuid,
) -> Result<Vec<FriendRowDto>, sqlx::Error> {
    let results = sqlx::query_as::<_, FriendRow>(
        r#"
        SELECT
            CASE WHEN f.user_id = $1 THEN f.friend_id ELSE f.user_id END AS friend_id,
            u.username,
            u.display_name
        FROM friends f
        JOIN users u
            ON u.id = CASE WHEN f.user_id = $1 THEN f.friend_id ELSE f.user_id END
        WHERE $1 IN (f.user_id, f.friend_id)
        ORDER BY u.username ASC
    "#,
    )
    .bind(current_user_id)
    .fetch_all(db)
    .await?;

    let friends = results
        .into_iter()
        .map(|row| FriendRowDto {
            friend_id: row.friend_id,
            username: row.username,
            display_name: row.display_name,
        })
        .collect();

    Ok(friends)
}

pub async fn get_friend_username(
    db: &PgPool,
    user_id: Uuid,
    receiver_user_id: Uuid,
) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar(
        r#"
        SELECT u.username
        FROM users u
        WHERE u.id = $1
            AND EXISTS(
                SELECT 1
                FROM friends
                WHERE
                    (user_id = $1 AND friend_id = $2)
                    OR
                    (user_id = $2 AND friend_id = $1)
            )
        "#,
    )
    .persistent(false)
    .bind(user_id)
    .bind(receiver_user_id)
    .fetch_optional(db)
    .await
}
