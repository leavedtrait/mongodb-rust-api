use mongodb::bson::{doc, Document};
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::{bson::oid::ObjectId, Database};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub _id: ObjectId,
    pub title: String,
    pub isbn: String,
    pub author: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookRequest {
    pub title: String,
    pub isbn: String,
    pub author: ObjectId,
}

impl TryFrom<BookRequest> for Book {
    type Error = String;
    fn try_from(value: BookRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            title: value.title,
            isbn: value.isbn,
            author: value.author,
        })
    }
}

impl Book {
    pub async fn insert_one(&self, db: Database) -> Result<InsertOneResult, String> {
        let result: InsertOneResult = db
            .collection::<Book>("books")
            .insert_one(self)
            .await
            .unwrap();
        Ok(result)
    }
    pub async fn update_one(&self, db: Database) -> Result<UpdateResult, String> {
        let query: mongodb::bson::Document = doc! {"_id": self._id};
        let updated_book: mongodb::bson::Document = doc! {
            "$set": {
                "title": self.title.clone(),
                "isbn": self.isbn.clone(),
                "author": self.author
            }
        };
        let result = db
            .collection::<Document>("books")
            .update_one(query, updated_book)
            .await
            .unwrap();
        Ok(result)
    }
}
