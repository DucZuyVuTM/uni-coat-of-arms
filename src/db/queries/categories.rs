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
pub async fn fetch_category_by_id(pool: &PgPool, id: i32) -> Result<Option<Category>, sqlx::Error> {
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
