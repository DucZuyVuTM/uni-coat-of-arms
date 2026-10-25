use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::db::queries;
use crate::models::{Category, CreateCategory};
use crate::state::AppState;

/// POST /categories - Create new category
pub async fn create_category(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCategory>,
) -> Result<Json<Category>, StatusCode> {
    let category = queries::insert_category(
        &state.db,
        &payload.name,
        payload.description.as_deref(),
    )
    .await
    .map_err(|e| {
        eprintln!("DB error: {}", e);
        // If name already exists (unique constraint) → 409 Conflict
        StatusCode::CONFLICT
    })?;

    Ok(Json(category))
}

/// GET /categories - List all categories
pub async fn list_categories(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Category>>, StatusCode> {
    let categories = queries::fetch_all_categories(&state.db)
        .await
        .map_err(|e| {
            eprintln!("DB error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(categories))
}

/// GET /categories/{id} - Get category by ID
pub async fn get_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Category>, StatusCode> {
    let category = queries::fetch_category_by_id(&state.db, id)
        .await
        .map_err(|e| {
            eprintln!("DB error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match category {
        Some(c) => Ok(Json(c)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// PUT /categories/{id} - Update an existing category
pub async fn update_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateCategory>,
) -> Result<Json<Category>, StatusCode> {
    let category = queries::update_category(
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

    Ok(Json(category))
}

/// DELETE /categories/{id} - Delete a category
pub async fn delete_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let rows_affected = queries::delete_category(&state.db, id)
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
