use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    Json,
};
use crate::firestore::{add_order, get_order, get_all_orders};
use crate::models::Order;
use firestore::FirestoreDb;
use std::sync::Arc;
use hyper::StatusCode;
use serde_json::json;
use tokio_stream::StreamExt;

/// Handler to create a new MTG card.
pub async fn create_order(
    Extension(firestore): Extension<Arc<FirestoreDb>>,
    Json(order): Json<Order>,
) -> impl IntoResponse {
    match add_order(&firestore, &order).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({ "message": "Card created successfully", "card": order })),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to create card: {:?}", err) })),
        ),
    }
}

/// Handler to retrieve a specific MTG order by ID.
pub async fn get_one_order(
    Extension(firestore): Extension<Arc<FirestoreDb>>,
    Path(order_id): Path<String>,
) -> impl IntoResponse {
    match get_order(&firestore, &order_id).await {
        Ok(Some(order)) => (
            StatusCode::OK,
            Json(json!({ "order": order })),
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Order not found" })),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to retrieve order: {:?}", err) })),
        ),
    }
}

/// Handler to retrieve all Orders.
pub async fn get_all_orders_handler(
    Extension(firestore): Extension<Arc<FirestoreDb>>,
) -> impl IntoResponse {
    match get_all_orders(&firestore).await {
        Ok(orders_stream) => {
            // Collect all orders from the stream into a vector
            let orders: Vec<Order> = orders_stream.collect().await;

            (
                StatusCode::OK,
                Json(json!({ "orders": orders })),
            )
        },
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to retrieve orders: {:?}", err) })),
        ),
    }
}

