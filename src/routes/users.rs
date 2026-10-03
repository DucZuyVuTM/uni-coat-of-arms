use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::handlers;
use crate::state::AppState;

/// Router for endpoint group /users
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", post(handlers::create_user))
        .route("/users", get(handlers::list_users))
        .route("/users/{id}", get(handlers::get_user))
}
