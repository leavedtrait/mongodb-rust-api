use actix_web::{get, post, web, HttpResponse, Responder};
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;

use crate::{
    db,
    models::book_models::{Book, BookRequest},
    AppState,
};

#[post("/create")]
pub async fn create_book(
    req: web::Json<BookRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    // Try to convert AuthorRequest into Author
    let book = Book::try_from(req.into_inner());

    match book {
        Ok(book) => {
            println!("Successfully created book: {:?}", book);
            let result = book
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
pub async fn get_book_by_id(
    path: web::Path<PathParam>,
    state: web::Data<AppState>,
) -> impl Responder {
    let oid = ObjectId::parse_str(path.id.as_str());
    match oid {
        Ok(id) => {
            let author = db::books::find_book_by_id(state.db.database("rust-bookstore"), id)
                .await
                .unwrap();
            let res = serde_json::to_string(&author).unwrap();
            HttpResponse::Ok().body(res)
        }
        Err(_) => HttpResponse::BadRequest().body("invalid id"),
    }
}

#[post("/update/{id}")]
pub async fn update_book(
    req: web::Json<BookRequest>,
    path: web::Path<PathParam>,
    state: web::Data<AppState>,
) -> impl Responder {
    // Try to convert AuthorRequest into Author
    let book = Book::try_from(req.into_inner());

    match book {
        Ok(mut book) => {
            book._id = ObjectId::parse_str(path.id.as_str()).unwrap();
            let result = book
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
pub async fn delete_book_by_id(
    path: web::Path<PathParam>,
    state: web::Data<AppState>,
) -> impl Responder {
    let oid = ObjectId::parse_str(path.id.as_str());
    match oid {
        Ok(id) => {
            let author = db::books::delete_book_by_id(state.db.database("rust-bookstore"), id)
                .await
                .unwrap();
            let res = serde_json::to_string(&author).unwrap();
            HttpResponse::Ok().body(res)
        }
        Err(_) => HttpResponse::BadRequest().body("invalid id"),
    }
}
