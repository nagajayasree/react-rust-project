mod handlers;
mod models;

use std::str::FromStr;

use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

#[tokio::main]
async fn main() {
    // 1. Connect to SQLite (creates tracker.db if it doesn't exist)
    let options = SqliteConnectOptions::from_str("sqlite://tracker.db")
        .expect("invalid database URL")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("failed to connect to database");

    // 2. Run migrations from ./migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");

    // 3. Routes
    let app = Router::new()
        .route("/health", get(health))
        .route(
            "/applications",
            get(handlers::list_applications).post(handlers::create_application),
        )
        .route(
            "/applications/{id}",
            get(handlers::get_application)
                .put(handlers::update_application)
                .delete(handlers::delete_application),
        )
        .with_state(pool); // every handler can now use State<SqlitePool>

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}