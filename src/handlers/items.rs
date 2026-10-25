use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::db::queries;
use crate::models::{Category, CreateItem, Item};
use crate::state::AppState;

/// POST /items - Add new item
pub async fn create_item(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateItem>,
) -> Result<Json<Item>, StatusCode> {
    let item = queries::insert_item(
        &state.db,
        payload.user_id,
        &payload.name,
        payload.description.as_deref(),
    )
    .await
    .map_err(|e| {
        eprintln!("DB error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(item))
}

/// GET /items - List all items
pub async fn list_items(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Item>>, StatusCode> {
    let items = queries::fetch_all_items(&state.db)
        .await
        .map_err(|e| {
            eprintln!("DB error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(items))
}

/// GET /items/{id} - Get item by ID
pub async fn get_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Item>, StatusCode> {
    let item = queries::fetch_item_by_id(&state.db, id)
        .await
        .map_err(|e| {
            eprintln!("DB error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match item {
        Some(i) => Ok(Json(i)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// PUT /items/{id} - Update an existing item
pub async fn update_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateItem>,
) -> Result<Json<Item>, StatusCode> {
    let item = queries::update_item(
        &state.db,
        id,
        &payload.name,
        payload.description.as_deref(),
    )
    .await
    .map_err(|e| {
        eprintln!("DB error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(item))
}

/// DELETE /items/{id} - Delete an item
pub async fn delete_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let rows_affected = queries::delete_item(&state.db, id)
        .await
        .map_err(|e| {
            eprintln!("DB error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if rows_affected > 0 {
        Ok(StatusCode::NO_CONTENT)  // Return 204 if success
    } else {
        Err(StatusCode::NOT_FOUND)  // Return 404 if ID not found
    }
}

// --------------------------- <Item-Category Table> ---------------------------

/// POST /items/{item_id}/categories/{category_id}
/// Assign Category to Item
pub async fn assign_category(
    State(state): State<Arc<AppState>>,
    Path((item_id, category_id)): Path<(i32, i32)>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = queries::assign_category_to_item(&state.db, item_id, category_id).await;

    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(sqlx::Error::Database(db_err)) => {
            // Get PostgreSQL error code ("23503" or "23505")
            let code = db_err.code().unwrap_or_default();
            
            match code.as_ref() {
                // 23503: Foreign Key Violation (Item or Category does not exist)
                "23503" => Err((
                    StatusCode::NOT_FOUND, 
                    "Item ID or Category ID does not exist".to_string()
                )),
                // 23505: Unique Violation (Already assigned this Category to this Item before)
                "23505" => Err((
                    StatusCode::CONFLICT, 
                    "This category is already assigned to the item".to_string()
                )),
                // Other Database errors
                _ => Err((
                    StatusCode::INTERNAL_SERVER_ERROR, 
                    format!("Database error: {}", db_err.message())
                )),
            }
        },
        // Connection error, timeout error,...
        Err(e) => {
            eprintln!("Unexpected DB error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Internal server error".to_string()
            ))
        }
    }
}

/// GET /items/{item_id}/categories
/// Get a list of categories for a specific item
pub async fn get_item_categories(
    State(state): State<Arc<AppState>>,
    Path(item_id): Path<i32>,
) -> Result<Json<Vec<Category>>, StatusCode> {
    let categories = queries::fetch_categories_for_item(&state.db, item_id)
        .await
        .map_err(|e| {
            eprintln!("DB error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // If the item does not exist, the query function still returns an empty array (no error)
    // So, simply returning that array is sufficient.
    Ok(Json(categories))
}

// --------------------------- </Item-Category Table> ---------------------------
