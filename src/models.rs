use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published_year: i32,
}

#[derive(Deserialize)]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub published_year: i32,
}