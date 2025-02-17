use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow, Error, query, query_as};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime
}

impl Comment {
    pub async fn create(pool: &PgPool, user_id: i32, post_id: i32, content: &str) -> Result<Self, Error> {
        let comment = query_as!(
            Comment,
            r#"
                INSERT INTO comments (user_id, post_id, content)
                VALUES($1, $2, $3)
                RETURNING id, user_id, post_id, content, created_at
            "#,
            user_id,
            post_id,
            content
        )
        .fetch_one(pool)
        .await?;

        Ok(comment)
    }

    pub async fn find_by_post_id(pool: &PgPool, post_id: i32) -> Result<Vec<Self>, Error> {
        let comments = query_as!(
            Comment,
            r#"
                SELECT id, user_id, post_id, content, created_at
                FROM comments
                WHERE post_id = $1
                ORDER BY created_at DESC
            "#,
            post_id
        )
        .fetch_all(pool)
        .await?;

        Ok(comments)
    }

    pub async fn find_by_user_id(pool: &PgPool, user_id: i32) -> Result<Vec<Self>, Error> {
        let comments = query_as!(
            Comment,
            r#"
                SELECT id, user_id, post_id, content, created_at
                FROM comments
                WHERE user_id = $1
                ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(comments)
    }

    pub async fn delete(&self, pool: &PgPool) -> Result<(), Error> {
        query!(
            r#"
                DELETE FROM comments
                WHERE id = $1
            "#,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
