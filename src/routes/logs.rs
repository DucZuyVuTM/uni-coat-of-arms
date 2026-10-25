use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::handlers;
use crate::state::AppState;

/// Router for logs
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/export", get(handlers::export_logs_csv))
}
