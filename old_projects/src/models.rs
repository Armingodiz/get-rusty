use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub published_year: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub published_year: i32,
}