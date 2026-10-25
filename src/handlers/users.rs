use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::db::queries;
use crate::models::{CreateUser, User};
use crate::state::AppState;

/// POST /users - Create new user
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    let user = queries::insert_user(
        &state.db,
        &payload.username,
        &payload.email
    )
    .await
    .map_err(|e| {
        eprintln!("DB error: {}", e);
        // If username/email already exists (unique constraint) → 409 Conflict
        StatusCode::CONFLICT
    })?;

    Ok(Json(user))
}

/// GET /users - List all users
pub async fn list_users(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let users = queries::fetch_all_users(&state.db)
        .await
        .map_err(|e| {
            eprintln!("DB error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(users))
}

/// GET /users/{id} - Get user by ID
pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    let user = queries::fetch_user_by_id(&state.db, id)
        .await
        .map_err(|e| {
            eprintln!("DB error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match user {
        Some(u) => Ok(Json(u)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// PUT /users/{id} - Update an existing user
pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    let user = queries::update_user(
        &state.db,
        id,
        &payload.username,
        &payload.email,
    )
    .await
    .map_err(|e| {
        eprintln!("DB error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(user))
}

/// DELETE /users/{id} - Delete a user
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let rows_affected = queries::delete_user(&state.db, id)
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
