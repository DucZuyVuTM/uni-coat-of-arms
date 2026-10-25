use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::handlers;
use crate::state::AppState;

/// Router for endpoint group /categories
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/categories", post(handlers::create_category))
        .route("/categories", get(handlers::list_categories))
        .route("/categories/{id}", get(handlers::get_category))
}
