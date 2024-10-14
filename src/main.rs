mod models;
mod handlers;
mod db;

use actix_web::{web, App, HttpServer};
use handlers::{get_books, create_book, get_book, update_book, delete_book};
use db::establish_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();  // Load environment variables from `.env`
    let pool = establish_connection().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::resource("/books")
                    .route(web::get().to(get_books))
                    .route(web::post().to(create_book))
            )
            .service(
                web::resource("/books/{id}")
                    .route(web::get().to(get_book))
                    .route(web::put().to(update_book))
                    .route(web::delete().to(delete_book))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}