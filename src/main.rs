mod models;
mod handlers;
mod db;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

use actix_web::{web, App, HttpServer};
use handlers::{get_books, create_book, get_book, update_book, delete_book};
use db::establish_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let final_sum = sum_array_concurrently(&array).await;

    println!("Final sum: {}", final_sum);
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



async fn sum_worker(worker_id: usize, data: Vec<i32>, sender_channel: mpsc::Sender<i32>) {
    if worker_id % 2 ==0 {
        sleep(Duration::from_secs(2)).await;
    }else{
        sleep(Duration::from_secs(1)).await;
    }
    let sum: i32 = data.iter().sum();
    println!("Worker {} sum is {}", worker_id, sum);

    sender_channel.send(sum).await.expect("Failed to send sum to main thread");
}


async fn sum_array_concurrently(array: &[i32]) -> i32{
    let slice_size = array.len() / 3;
    let slice1 = array[0..slice_size].to_vec();
    let slice2 = array[slice_size..(2 * slice_size)].to_vec();
    let slice3 = array[(2 * slice_size)..].to_vec();
    let (tx, mut rx) = mpsc::channel(3);

    /*
    I had to clone the tx channel to be able to pass it to the tokio::spawn function, Because the tx channel is moved to the spawned function and it is not possible to use it again.
    Also tx does not implement the Copy trait, so I had to clone it.
     */
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    /*
    I first decleared slice like below: 
    let slice1 = &array[0..slice_size];
    then when wanted to pass it to the sum_worker function, I got an error that the slice does not live long enough.
    It was because the slice was not owned by the function. So I had to change it to a vector by calling to_vec() method on the slice.
     */
    tokio::spawn(sum_worker(1, slice1, tx));
    tokio::spawn(sum_worker(2, slice2, tx1));
    tokio::spawn(sum_worker(3, slice3, tx2));
    let mut total_sum = 0;
    for _ in 0..3 {
        if let Some(slice_sum) = rx.recv().await {
            total_sum += slice_sum;
        }
    }
    
    total_sum
}

