pub mod authors;
pub mod books;

use mongodb::Client;
use std::env;

use mongodb::{
    bson::Document,
    results::{InsertOneResult, UpdateResult},
    Database,
};
use serde::{Deserialize, Serialize};

pub async fn connect_to_db() -> mongodb::Client {
    // Get the MongoDB URI from the environment variable, if set
    let mongo_uri =
        env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    // Create a new client and connect to the server
    let client = Client::with_uri_str(mongo_uri).await.unwrap();
    client
}

pub trait MongoDBOperations {
    fn insert_one<T>(
        doc: T,
        db: Database,
        collection_name: String,
    ) -> impl std::future::Future<Output = Result<InsertOneResult, String>> + Send
    where
        T: Sync + Send + Serialize + for<'a> Deserialize<'a>,
    {
        async move {
            let result: InsertOneResult = db
                .collection::<T>(&collection_name)
                .insert_one(doc)
                .await
                .unwrap();
            Ok(result)
        }
    }
    fn update_one(
        doc: Document,
        query: Document,
        db: Database,
        collection_name: String,
    ) -> impl std::future::Future<Output = Result<UpdateResult, String>> + Send {
        async move {
            let result: UpdateResult = db
                .collection::<Document>(&collection_name)
                .update_one(query, doc)
                .await
                .unwrap();
            Ok(result)
        }
    }
}
