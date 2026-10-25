use sqlx::PgPool;

use crate::models::Category;

/// CREATE: Add new category
pub async fn insert_category(
    pool: &PgPool,
    name: &str,
    description: Option<&str>,
) -> Result<Category, sqlx::Error> {
    sqlx::query_as::<_, Category>(
        r#"
        INSERT INTO items (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description, created_at
        "#,
    )
    .bind(name)
    .bind(description)
    .fetch_one(pool)
    .await
}

/// READ: List all items
pub async fn fetch_all_categories(pool: &PgPool) -> Result<Vec<Category>, sqlx::Error> {
    sqlx::query_as::<_, Category>(
        r#"
        SELECT id, name, description, created_at
        FROM items
        ORDER BY id
        "#,
    )
    .fetch_all(pool)
    .await
}

/// READ: Get item by id
pub async fn fetch_category_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<Option<Category>, sqlx::Error> {
    sqlx::query_as::<_, Category>(
        r#"
        SELECT id, name, description, created_at
        FROM items
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

/// UPDATE: Update category information
pub async fn update_category(
    pool: &PgPool,
    id: i32,
    name: &str,
    description: Option<&str>,
) -> Result<Category, sqlx::Error> {
    sqlx::query_as::<_, Category>(
        r#"
        UPDATE categories
        SET name = $1, description = $2
        WHERE id = $3
        RETURNING id, name, description, created_at
        "#,
    )
    .bind(name)
    .bind(description)
    .bind(id)
    .fetch_one(pool)
    .await
}

/// DELETE: Delete category from table
pub async fn delete_category(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM categories
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
