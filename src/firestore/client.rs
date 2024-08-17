use firestore::{FirestoreDb, errors::FirestoreError, FirestoreDbOptions};
use std::env;
use dotenv::dotenv;

/// Initializes and returns a Firestore database client.
pub async fn init_firestore_client() -> Result<FirestoreDb, FirestoreError> {
    dotenv().ok();
    let project_id = env::var("GCP_PROJECT_ID").expect("GCP_PROJECT_ID must be set");

    FirestoreDb::with_options(FirestoreDbOptions::new(project_id)
            .with_database_id("orders".to_string())).await
}
