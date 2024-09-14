pub mod db;
pub mod handlers;
pub mod models;

use actix_web::{web, App, HttpServer};
use db::connect_to_db;
use handlers::authors;


pub struct AppState {
    db: mongodb::Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: mongodb::Client = connect_to_db().await;
    HttpServer::new(move || {
        let authors_scope = web::scope("/authors")
        .service(authors::delete_author_by_id)
        .service(authors::update_author)
        .service(authors::get_author_by_id)
        .service(authors::create_author);
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .service(authors_scope)
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
