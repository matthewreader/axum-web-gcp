use axum::Router;
use crate::routes::{orders, health};

/// Combines all routes into a single Router.
pub fn create_routes() -> Router {
    Router::new()
        .nest("/api", orders::order_router(true))
        .merge(health::health_router())
}