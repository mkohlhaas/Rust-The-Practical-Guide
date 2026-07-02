use std::time::Duration;
use tokio::time::sleep;
async fn printing(i: i32) {
    sleep(Duration::from_secs(1)).await;
    println!("Task {i}");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
...
}