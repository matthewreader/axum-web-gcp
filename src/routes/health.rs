use axum::{Router, routing::get};

use crate::handlers::health_handler;
pub fn router() -> Router {
    Router::new().route("/health", get(health_handler::health_check))
}
