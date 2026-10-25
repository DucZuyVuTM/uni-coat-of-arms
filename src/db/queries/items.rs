use sqlx::PgPool;

use crate::models::{Category, Item};

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

/// UPDATE: Update item information
pub async fn update_item(
    pool: &PgPool,
    id: i32,
    name: &str,
    description: Option<&str>,
) -> Result<Item, sqlx::Error> {
    sqlx::query_as::<_, Item>(
        r#"
        UPDATE items
        SET name = $1, description = $2
        WHERE id = $3
        RETURNING id, user_id, name, description, created_at
        "#,
    )
    .bind(name)
    .bind(description)
    .bind(id)
    .fetch_one(pool)
    .await
}

/// DELETE: Delete item from table
pub async fn delete_item(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM items
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

// --------------------------- <Item-Category Table> ---------------------------

/// Create new table record - assign category to item
pub async fn assign_category_to_item(
    pool: &PgPool,
    item_id: i32,
    category_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO item_categories (item_id, category_id)
        VALUES ($1, $2)
        "#,
    )
    .bind(item_id)
    .bind(category_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Get a list of categories for a specific item
pub async fn fetch_categories_for_item(
    pool: &PgPool,
    item_id: i32,
) -> Result<Vec<Category>, sqlx::Error> {
    // Use JOIN to get category information through junction table
    sqlx::query_as::<_, Category>(
        r#"
        SELECT c.id, c.name, c.description, c.created_at
        FROM categories c
        JOIN item_categories ic ON c.id = ic.category_id
        WHERE ic.item_id = $1
        "#,
    )
    .bind(item_id)
    .fetch_all(pool)
    .await
}

// --------------------------- </Item-Category Table> ---------------------------
