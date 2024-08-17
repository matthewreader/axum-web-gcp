use firestore::{FirestoreDb, errors::FirestoreError, FirestoreDbOptions};
use std::env;
use dotenv::dotenv;
use std::fs;

/// Initializes and returns a Firestore database client.
pub async fn init_firestore_client() -> Result<FirestoreDb, FirestoreError> {
    dotenv().ok();
    let project_id = env::var("GCP_PROJECT_ID").expect("GCP_PROJECT_ID must be set");
    let service_account_key = env::var("FIREBASE_SERVICE_ACCOUNT").expect("FIREBASE_SERVICE_ACCOUNT must be set");

    if !fs::metadata(&service_account_key).is_ok() {
        panic!("Service account key file not found: {}", service_account_key);
    }

    FirestoreDb::with_options_service_account_key_file(
        FirestoreDbOptions::new(project_id)
            .with_database_id("orders".to_string()), service_account_key.into()
    ).await
}
