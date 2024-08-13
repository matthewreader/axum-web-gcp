use axum::{Router, routing::get};

use crate::handlers::health_handler;
pub fn health_router() -> Router {
    Router::new()
        .route("/health", get(health_handler::health_check))
}
