use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::handlers;
use crate::state::AppState;

/// Router for endpoint group /items
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/items", post(handlers::create_item))
        .route("/items", get(handlers::list_items))
        .route("/items/{id}", get(handlers::get_item))
}
