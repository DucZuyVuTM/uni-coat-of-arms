mod db;
mod handlers;
mod models;
mod routes;
mod state;

use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get URL CSDL from env
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Create database and create table if not exists
    db::init_db(&pool).await?;

    // Create state and put it into Arc
    let state = Arc::new(AppState { db: pool });

    // Create router
    let app = routes::create_router(state);

    // Run server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Сборка и запуск произведены. Автор: Ву Дык Зуй");
    axum::serve(listener, app).await?;

    Ok(())
}
