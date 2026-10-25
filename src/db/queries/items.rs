use sqlx::PgPool;

use crate::models::Item;

/// CREATE: Add new item
pub async fn insert_item(
    pool: &PgPool,
    user_id: Option<i32>,
    name: &str,
    description: Option<&str>,
) -> Result<Item, sqlx::Error> {
    sqlx::query_as::<_, Item>(
        r#"
        INSERT INTO items (user_id, name, description)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, name, description, created_at
        "#,
    )
    .bind(user_id)
    .bind(name)
    .bind(description)
    .fetch_one(pool)
    .await
}

/// READ: List all items
pub async fn fetch_all_items(pool: &PgPool) -> Result<Vec<Item>, sqlx::Error> {
    sqlx::query_as::<_, Item>(
        r#"
        SELECT id, user_id, name, description, created_at
        FROM items
        ORDER BY id
        "#,
    )
    .fetch_all(pool)
    .await
}

/// READ: Get item by id
pub async fn fetch_item_by_id(pool: &PgPool, id: i32) -> Result<Option<Item>, sqlx::Error> {
    sqlx::query_as::<_, Item>(
        r#"
        SELECT id, user_id, name, description, created_at
        FROM items
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}
