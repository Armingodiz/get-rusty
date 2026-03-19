use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

async fn sum_worker(worker_id: usize, data: Vec<i32>, sender_channel: mpsc::Sender<i32>) {
    if worker_id % 2 == 0 {
        sleep(Duration::from_secs(2)).await;
    } else {
        sleep(Duration::from_secs(1)).await;
    }
    let sum: i32 = data.iter().sum();
    println!("Worker {} sum is {}", worker_id, sum);

    sender_channel
        .send(sum)
        .await
        .expect("Failed to send sum to main thread");
}

pub async fn sum_array_concurrently(array: &[i32]) -> i32 {
    let slice_size = array.len() / 3;
    let slice1 = array[0..slice_size].to_vec();
    let slice2 = array[slice_size..(2 * slice_size)].to_vec();
    let slice3 = array[(2 * slice_size)..].to_vec();
    let (tx, mut rx) = mpsc::channel(3);

    let tx1 = tx.clone();
    let tx2 = tx.clone();

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

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_sum_array_concurrently() {
        let array = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let sum = sum_array_concurrently(&array).await;
        assert_eq!(sum, 45);
    }
}