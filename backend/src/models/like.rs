use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow, Error, query, query_as};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Like {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
    pub created_at: NaiveDateTime,
}

impl Like {
    pub async fn create(pool: &PgPool, user_id: i32, post_id: i32) -> Result<Self, Error> {
        let like = query_as!(
            Like,
            r#"
                INSERT INTO likes (user_id, post_id)
                VALUES ($1, $2)
                RETURNING id, user_id, post_id, created_at
            "#,
            user_id,
            post_id
        )
        .fetch_one(pool)
        .await?;

        Ok(like)
    }
    pub async fn find(pool: &PgPool, user_id: i32, post_id: i32) -> Result<Option<Self>, Error> {
        let like = query_as!(
            Like,
            r#"
                SELECT id, user_id, post_id, created_at
                FROM likes
                WHERE user_id = $1 AND post_id = $2
            "#,
            user_id,
            post_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(like)
    }
    pub async fn delete(&self, pool: &PgPool) -> Result<(), Error> {
        query!(
            r#"
                DELETE FROM likes
                WHERE id = $1
            "#,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}