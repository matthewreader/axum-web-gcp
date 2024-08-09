use axum::{response::IntoResponse, Json};
use axum::http::StatusCode;
use serde_json::{json, Value};
use chrono::Utc;

pub async fn health_check() -> impl IntoResponse {
    let dtm = Utc::now();
    let response: Value = json!({
        "status": "success",
        "time": dtm
    });

    // Return a tuple containing the status code and the JSON response
    (StatusCode::OK, Json(response))
}
