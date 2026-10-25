use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

use crate::handlers;
use crate::state::AppState;

/// Router for endpoint group /items
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(handlers::create_item))
        .route("/", get(handlers::list_items))
        .route("/:id", get(handlers::get_item))
        .route("/:id", put(handlers::update_item))
        .route("/:id", delete(handlers::delete_item))

        // --- Routes for item's category ---
        .route("/:item_id/categories/:category_id", post(handlers::items::assign_category))
        .route("/:item_id/categories", get(handlers::items::get_item_categories))
        // ----------------------------------
}
