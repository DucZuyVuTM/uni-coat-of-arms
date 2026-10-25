use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

use crate::handlers;
use crate::state::AppState;

/// Router for endpoint group /categories
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(handlers::create_category))
        .route("/", get(handlers::list_categories))
        .route("/:id", get(handlers::get_category))
        .route("/:id", put(handlers::update_category))
        .route("/:id", delete(handlers::delete_category))
}
