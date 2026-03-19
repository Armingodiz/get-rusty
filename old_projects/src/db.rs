use sqlx::{SqlitePool, Pool, sqlite::Sqlite};

pub type DbPool = Pool<Sqlite>;

pub async fn establish_connection() -> DbPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    SqlitePool::connect(&database_url).await
        .expect("Failed to connect to the database")
}