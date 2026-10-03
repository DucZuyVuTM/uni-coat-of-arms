use sqlx::PgPool;

use crate::models::Item;

/// Create new item
pub async fn insert_item(pool: &PgPool, name: &str) -> Result<Item, sqlx::Error> {
    sqlx::query_as::<_, Item>(
        "INSERT INTO items (name) VALUES ($1) RETURNING id, name",
    )
    .bind(name)
    .fetch_one(pool)
    .await
}

/// List all items
pub async fn fetch_all_items(pool: &PgPool) -> Result<Vec<Item>, sqlx::Error> {
    sqlx::query_as::<_, Item>("SELECT id, name FROM items ORDER BY id")
        .fetch_all(pool)
        .await
}

/// Get item by id
pub async fn fetch_item_by_id(pool: &PgPool, id: i32) -> Result<Option<Item>, sqlx::Error> {
    sqlx::query_as::<_, Item>("SELECT id, name FROM items WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}
