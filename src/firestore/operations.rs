use firestore::{FirestoreDb, errors::FirestoreError};
use crate::models::Order;
use futures::stream::BoxStream;

/// Adds a new order to Firestore.
pub async fn add_order(
    firestore: &FirestoreDb,
    order: &Order,
) -> Result<(), FirestoreError> {
    firestore
        .fluent()
        .insert()
        .into("orders")
        .document_id(&order.order_id)
        .object(order)
        .execute()
        .await?;
    Ok(())
}

/// Retrieves an order by ID from Firestore.
/// TODO: Investigate why this function does not correctly return an `Order` object.
pub async fn get_order(
    firestore: &FirestoreDb,
    order_id: &str,
) -> Result<Option<Order>, FirestoreError> {
    let order: Option<Order> = firestore
        .fluent()
        .select()
        .by_id_in("orders")
        .obj()
        .one(&order_id)
        .await?;
    Ok(order)
}

/// Retrieves all orders from Firestore.
pub async fn get_all_orders(
    firestore: &FirestoreDb,
) -> Result<BoxStream<Order>, FirestoreError> {
    let orders: BoxStream<Order> = firestore
        .fluent()
        .select()
        .from("orders")
        .obj()
        .stream_query()
        .await?;
    Ok(orders)
}
