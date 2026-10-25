mod db;
mod handlers;
mod models;
mod routes;
mod state;

use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::time::Duration;
use tracing_gelf::Logger;

use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // --------------------- <GrayLog> ---------------------

    // Initialize GELF Logger and Spawn task
    let graylog_addr = "graylog:12201";
    let mut conn_handle = Logger::builder().init_udp(graylog_addr).unwrap();
    tokio::spawn(async move {
        conn_handle.connect().await;
    });

    // Wait for 1 sec until UDP connection is set
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Send first log
    tracing::info!("Rust App is on and is sending logs to Graylog!");

    // --------------------- </GrayLog> ---------------------


    // --------------------- <DBMS> ---------------------

    // Get URL DBMS from env
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

    // --------------------- </DBMS> ---------------------


    // --------------------- <App> ---------------------

    // Create router
    let app = routes::create_router(state);

    // Run server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Сборка и запуск произведены. Автор: Ву Дык Зуй");
    axum::serve(listener, app).await?;

    // --------------------- </App> ---------------------

    Ok(())
}
