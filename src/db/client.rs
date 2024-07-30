use mongodb::{Client, Database};
use std::env;
use dotenv::dotenv;

pub async fn get_db() -> Database {
    dotenv().ok();  

  let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let client = Client::with_uri_str(&mongo_uri)
        .await
        .expect("Failed to initialize MongoDB client");

    client.database("test_rust")
}
