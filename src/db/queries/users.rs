use sqlx::PgPool;

use crate::models::User;

/// Create new user
pub async fn insert_user(
    pool: &PgPool,
    username: &str,
    email: &str,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email",
    )
    .bind(username)
    .bind(email)
    .fetch_one(pool)
    .await
}

/// List all users
pub async fn fetch_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT id, username, email FROM users ORDER BY id")
        .fetch_all(pool)
        .await
}

/// Get user by id
pub async fn fetch_user_by_id(pool: &PgPool, id: i32) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT id, username, email FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}
