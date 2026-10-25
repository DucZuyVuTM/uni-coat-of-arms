use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::db::queries;
use crate::models::{CreateItem, Item};
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
