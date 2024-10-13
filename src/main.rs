async fn say_hello() {
    println!("hello, world!");
}

#[tokio::main]
async fn main() {
    say_hello().await;
}