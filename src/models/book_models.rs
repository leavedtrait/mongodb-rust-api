use std::result;

use mongodb::bson::doc;
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::{bson::oid::ObjectId, Database};

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
    pub async fn update_one(&self , db:Database) -> Result<UpdateResult,string>{
        let query: mongodb::bson::Document = doc! {"_id": self._id};
        let updated_book: mongodb::bson::Document = doc! {
            "$set": {
                "title":self.title,
                "isbn":self.isbn,
                "author":self.author
            }
        };
        let result = db.collection("books").update_one(query, updated_book);
        Ok(result)
    }
}
