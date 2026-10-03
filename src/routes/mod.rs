pub mod items;
pub mod users;

use axum::Router;
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::state::AppState;

/// Root router that merges all child routers
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(items::router())
        .merge(users::router())
        .nest_service("/mirea_gerb", ServeDir::new("static"))
        .with_state(state)
}
