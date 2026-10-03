pub mod queries;

use sqlx::PgPool;

/// Create database if not exists
pub async fn init_db(pool: &PgPool) -> anyhow::Result<()> {
    // Table Items
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS items (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Table Users
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
