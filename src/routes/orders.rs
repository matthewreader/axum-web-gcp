use axum::{Router, routing::get};
use axum::routing::post;
use crate::handlers::order_handler::{create_order, get_one_order, get_all_orders_handler};

pub fn order_router(db: bool) -> Router {
    Router::new()
        .route("/orders/:order_id", get(get_one_order))
        .route("/orders", get(get_all_orders_handler))
        .route("/orders", post(create_order))
        .layer(axum::Extension(db))
}