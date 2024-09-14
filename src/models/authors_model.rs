use mongodb::{
    bson::{doc, oid::ObjectId},
    results::{InsertOneResult, UpdateResult},
    Database,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub _id: ObjectId,
    pub firstname: String,
    pub lastname: String,
    pub phone_number: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorRequest {
    pub firstname: String,
    pub lastname: String,
    pub phone_number: i64,
}

impl TryFrom<AuthorRequest> for Author {
    type Error = String;

    fn try_from(value: AuthorRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            firstname: value.firstname,
            lastname: value.lastname,
            phone_number: value.phone_number,
        })
    }
}

impl Author {
    pub async fn insert_one(&self, db_client: Database) -> Result<InsertOneResult, String> {
        let result = db_client
            .collection::<Author>("authors")
            .insert_one(self)
            .await
            .unwrap();
        Ok(result)
    }
    pub async fn update_one(&self, db_client: Database) -> Result<UpdateResult, String> {
        let filter = doc! {"_id": self._id};
        let updated_author = doc! {
            "$set": {
                "firstname": self.firstname.clone(),
                "lastname": self.lastname.clone(),
                "phone_number": self.phone_number,
            }
        };
        let result = db_client
            .collection::<Author>("authors")
            .update_one(filter, updated_author)
            .await
            .unwrap();

        Ok(result)
    }
}
