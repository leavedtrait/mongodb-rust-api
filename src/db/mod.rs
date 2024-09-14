pub mod authors;


use mongodb::Client;
use std::env;

pub async fn connect_to_db() -> mongodb::Client {
    // Get the MongoDB URI from the environment variable, if set
    let mongo_uri = env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    // Create a new client and connect to the server
    let client = Client::with_uri_str(mongo_uri).await.unwrap();
    client
}
