pub mod queries;

use sqlx::PgPool;

/// Create database if not exists
pub async fn init_db(pool: &PgPool) -> anyhow::Result<()> {
    // Table Categories
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS categories (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL UNIQUE,
            description TEXT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Table Items
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS items (
            id SERIAL PRIMARY KEY,
            user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            description TEXT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
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
            email TEXT NOT NULL UNIQUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Junction Table Item_categories
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS item_categories (
            item_id INTEGER REFERENCES items(id) ON DELETE CASCADE,
            category_id INTEGER REFERENCES categories(id) ON DELETE CASCADE,
            PRIMARY KEY (item_id, category_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
