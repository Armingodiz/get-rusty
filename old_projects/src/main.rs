use actix_web::{web, App, HttpServer};
use dotenv;
use tokio;
use tonic::transport::Server;
mod models;
mod handlers;
use handlers::{get_books, create_book, get_book, update_book, delete_book};
mod db;
use db::establish_connection;

mod server;
use server::MyGreeter;

mod proto;
use proto::hello_world;
use hello_world::greeter_server::GreeterServer;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    let grpc_server = Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr);

    dotenv::dotenv().ok(); // Load environment variables from `.env`
    let pool = establish_connection().await;

    let actix_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::resource("/books")
                    .route(web::get().to(get_books))
                    .route(web::post().to(create_book)),
            )
            .service(
                web::resource("/books/{id}")
                    .route(web::get().to(get_book))
                    .route(web::put().to(update_book))
                    .route(web::delete().to(delete_book)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run();

    // **Run both servers concurrently**
    let (grpc_result, actix_result) = tokio::join!(grpc_server, actix_server);

    // **Handle any errors**
    grpc_result?;
    actix_result?;

    Ok(())
}