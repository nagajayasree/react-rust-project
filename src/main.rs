mod models;

use axum::{routing::get, Json, Router};
use models::{Application, Status};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/applications", get(list_applications));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

// Hardcoded for today — tomorrow this comes from SQLite
async fn list_applications() -> Json<Vec<Application>> {
    let apps = vec![
        Application { id: 1, company: "Microsoft".into(), role: "Frontend Engineer".into(), status: Status::Applied },
        Application { id: 2, company: "Amazon".into(), role: "UI Engineer".into(), status: Status::Interview },
        Application { id: 3, company: "Expedia".into(), role: "React Developer".into(), status: Status::Offer },
        Application { id: 4, company: "Zillow".into(), role: "UX Engineer".into(), status: Status::Rejected },
    ];
    Json(apps)
}