use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user_model::User;

pub async fn get_user_by_id(db: &PgPool, id: &Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(r#"SELECT * from users WHERE id = $1"#)
        .bind(id)
        .fetch_optional(db)
        .await
}
