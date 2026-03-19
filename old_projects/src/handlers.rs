use actix_web::{web, HttpResponse, Responder, Error};
use crate::models::{Book, CreateBook};
use crate::db::DbPool;
use sqlx::query_as;


pub async fn get_book(
    pool: web::Data<DbPool>,
    book_id: web::Path<i64>,
) -> Result<HttpResponse, Error> {
    let book_id = book_id.into_inner();
    let book = query_as!(Book, "SELECT * FROM books WHERE id = ?", book_id)
        .fetch_one(pool.get_ref())
        .await
        .expect("Error loading book");

    Ok(HttpResponse::Ok().json(book))
}

pub async fn get_books(pool: web::Data<DbPool>) -> impl Responder {
    let books = query_as!(Book, "SELECT * FROM books")
        .fetch_all(pool.get_ref())
        .await
        .expect("Error loading books");
    HttpResponse::Ok().json(books)
}

pub async fn create_book(pool: web::Data<DbPool>, book: web::Json<CreateBook>) -> impl Responder {
    let _ = sqlx::query!(
        "INSERT INTO books (title, author, published_year) VALUES (?, ?, ?)",
        book.title, book.author, book.published_year
    )
    .execute(pool.get_ref())
    .await
    .expect("Error inserting book");

    HttpResponse::Created().json(book.into_inner())
}

pub async fn update_book(pool: web::Data<DbPool>, book_id: web::Path<i32>, book: web::Json<CreateBook>) -> impl Responder {
    let book_id = book_id.into_inner();
    let _ = sqlx::query!(
        "UPDATE books SET title = ?, author = ?, published_year = ? WHERE id = ?",
        book.title, book.author, book.published_year, book_id)
    .execute(pool.get_ref())
    .await
    .expect("Error updating book");

    HttpResponse::Ok().json(book.into_inner())
}

pub async fn delete_book(pool: web::Data<DbPool>, book_id: web::Path<i32>) -> impl Responder {
    let book_id = book_id.into_inner();
    let _ = sqlx::query!("DELETE FROM books WHERE id = ?", book_id)
        .execute(pool.get_ref())
        .await
        .expect("Error deleting book");

    HttpResponse::NoContent().finish()
}