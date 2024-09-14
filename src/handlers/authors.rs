use actix_web::{get, post, web, HttpResponse, Responder};
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;

use crate::{
    db,
    models::authors_model::{Author, AuthorRequest},
    AppState,
};

#[post("/create")]
pub async fn create_author(
    req: web::Json<AuthorRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    // Try to convert AuthorRequest into Author
    let author = Author::try_from(req.into_inner());

    match author {
        Ok(author) => {
            println!("Successfully created Author: {:?}", author);
            let result = author
                .insert_one(state.db.database("rust-bookstore"))
                .await
                .unwrap();
            let res = serde_json::to_string(&result).unwrap();
            HttpResponse::Created().body(res)
        }
        Err(e) => {
            println!("Failed to create Author: {}", e);
            HttpResponse::InternalServerError().body(e)
        }
    }
}

#[derive(Debug, Deserialize)]
struct PathParam {
    id: String,
}
#[get("/{id}")]
pub async fn get_author_by_id(
    path: web::Path<PathParam>,
    state: web::Data<AppState>,
) -> impl Responder {
    let oid = ObjectId::parse_str(path.id.as_str());
    match oid {
        Ok(id) => {
            let author = db::authors::find_author_by_id(state.db.database("rust-bookstore"), id)
                .await
                .unwrap();
            let res = serde_json::to_string(&author).unwrap();
            HttpResponse::Ok().body(res)
        }
        Err(_) => HttpResponse::BadRequest().body("invalid id"),
    }
}

#[post("/update/{id}")]
pub async fn update_author(
    req: web::Json<AuthorRequest>,
    path: web::Path<PathParam>,
    state: web::Data<AppState>,
) -> impl Responder {
    // Try to convert AuthorRequest into Author
    let author = Author::try_from(req.into_inner());

    match author {
        Ok(mut author) => {
            author._id = ObjectId::parse_str(path.id.as_str()).unwrap();
            let result = author
                .update_one(state.db.database("rust-bookstore"))
                .await
                .unwrap();
            let res = serde_json::to_string(&result).unwrap();
            HttpResponse::Ok().body(res)
        }
        Err(e) => {
            println!("Failed to get Author: {}", e);
            HttpResponse::InternalServerError().body(e)
        }
    }
}

#[post("/delete/{id}")]
pub async fn delete_author_by_id(
    path: web::Path<PathParam>,
    state: web::Data<AppState>,
) -> impl Responder {
    let oid = ObjectId::parse_str(path.id.as_str());
    match oid {
        Ok(id) => {
            let author = db::authors::delete_author_by_id(state.db.database("rust-bookstore"), id)
                .await
                .unwrap();
            let res = serde_json::to_string(&author).unwrap();
            HttpResponse::Ok().body(res)
        }
        Err(_) => HttpResponse::BadRequest().body("invalid id"),
    }
}
