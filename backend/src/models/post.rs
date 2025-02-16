use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow, Error, query, query_as};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl Post {
    pub async fn create(
        pool: &PgPool,
        user_id: i32,
        title: &str,
        content: &str,
    ) -> Result<Self, Error> {
        let post = query_as!(
            Post,
            r#"
                INSERT INTO posts (user_id, title, content)
                VALUES ($1, $2, $3)
                RETURNING id, user_id, title, content, created_at, updated_at
            "#,
            user_id,
            title,
            content
        ).fetch_one(pool).await?;

        Ok(post)
    }
    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Self, Error> {
        let post = query_as!(
            Post,
            r#"
                SELECT id, user_id, title, content, created_at, updated_at 
                FROM posts
                WHERE id = $1
            "#,
            id
        ).fetch_one(pool).await?;

        Ok(post)
    }
    pub async fn find_all(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT id, user_id, title, content, created_at, updated_at
            FROM posts
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;
    
        Ok(posts)
    }
    pub async fn find_by_user_id(pool: &PgPool, user_id: i32) -> Result<Vec<Self>, Error> {
        let posts = query_as!(
            Post,
            r#"
                SELECT id, user_id, title, content, created_at, updated_at 
                FROM posts
                WHERE user_id = $1
                ORDER BY created_at DESC
            "#,
            user_id
        ).fetch_all(pool).await?;

        Ok(posts)
    }
    pub async fn update(&self, pool: &PgPool, title: &str, content: &str) -> Result<Self, Error> {
        let post = query_as!(
            Post,
            r#"
                UPDATE posts
                SET title = $1, content = $2, updated_at = CURRENT_TIMESTAMP
                WHERE id = $3
                RETURNING id, user_id, title, content, created_at, updated_at
            "#,
            title,
            content,
            self.id
        )
        .fetch_one(pool)
        .await?;

        Ok(post)
    }
    pub async fn delete(&self, pool: &PgPool) -> Result<(), Error> {
        query!(
            r#"
                DELETE FROM posts
                WHERE id = $1
            "#,
            self.id
        ).execute(pool).await?;

        Ok(())
    }
}