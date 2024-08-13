use std::net::SocketAddr;
use std::sync::Arc;
use dotenv::dotenv;
use firestore::init_firestore_client;
use crate::routes::init::create_routes;

mod routes;
mod handlers;
mod state;
mod models;
mod firestore;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from the .env file
    dotenv().ok();

    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Initialize Firestore client
    let firestore_client = init_firestore_client()
        .await
        .expect("Failed to initialize Firestore client");

    // Build our application with routes
    let app = create_routes()
        .layer(axum::Extension(Arc::new(firestore_client)));

    // Run our app with hyper on the default address
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
