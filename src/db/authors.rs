use mongodb::{bson::{doc, oid::ObjectId}, results::DeleteResult, Database};

use crate::models::authors_model::Author;

pub async fn find_author_by_id(db_client: Database ,oid : ObjectId) -> Result<Author, String> {
    let result = db_client
        .collection::<Author>("authors")
        .find_one(doc! {"_id": oid})
        .await
        .unwrap();
    Ok(result.unwrap())
}

pub async fn delete_author_by_id(db_client: Database ,oid : ObjectId) -> Result<DeleteResult, String> {
    let result = db_client
        .collection::<Author>("authors")
        .delete_one(doc! {"_id": oid})
        .await
        .unwrap();
    Ok(result)
}