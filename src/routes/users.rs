use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

use crate::handlers;
use crate::state::AppState;

/// Router for endpoint group /users
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(handlers::create_user))
        .route("/", get(handlers::list_users))
        .route("/:id", get(handlers::get_user))
        .route("/:id", put(handlers::update_user))
        .route("/:id", delete(handlers::delete_user))
}
