pub mod categories;
pub mod items;
pub mod logs;
pub mod users;

use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::state::AppState;

/// Root router that merges all child routers
pub fn create_router(state: Arc<AppState>) -> Router {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    Router::new()
        .nest("/categories", categories::router())
        .nest("/items", items::router())
        .nest("/logs", logs::router())
        .nest("/users", users::router())
        .nest_service("/mirea_gerb", ServeDir::new("static"))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer)
        .with_state(state)
}
