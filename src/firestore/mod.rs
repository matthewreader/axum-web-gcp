pub mod client;
pub mod operations;

pub use client::init_firestore_client;
pub use operations::{add_order, get_order, get_all_orders};
