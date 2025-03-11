use axum::{response::IntoResponse, routing::get, Json, Router};
use serde_json;

pub async fn health_check() -> impl IntoResponse {
    let response = serde_json::json!({
        "status": "ok",
        "message": "Server is running"  
    });
    Json(response)
}

pub fn new_healthcheck_routes() -> Router {
    Router::new()
        .route("/", get(health_check))
}
