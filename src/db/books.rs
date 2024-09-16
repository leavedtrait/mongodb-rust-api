use mongodb::{
    bson::{doc, oid::ObjectId},
    results::DeleteResult,
    Database,
};

use crate::models::book_models::Book;

pub async fn find_book_by_id(db_client: Database, oid: ObjectId) -> Result<Book, String> {
    let result = db_client
        .collection::<Book>("books")
        .find_one(doc! {"_id": oid})
        .await
        .unwrap();
    Ok(result.unwrap())
}

pub async fn delete_book_by_id(db_client: Database, oid: ObjectId) -> Result<DeleteResult, String> {
    let result = db_client
        .collection::<Book>("books")
        .delete_one(doc! {"_id": oid})
        .await
        .unwrap();
    Ok(result)
}
