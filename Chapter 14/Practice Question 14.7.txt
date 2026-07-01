use tokio::time::{sleep, Duration};
async fn fn1() {
    println!("Task 1 started!");
    sleep(Duration::from_secs(3)).await;
    println!("Task 1 completed!");
}
async fn fn2() {
    println!("Task 2 started!");
    sleep(Duration::from_secs(2)).await;
    println!("Task 2 completed!");
}
#[tokio::main]
async fn main() {
    fn1().await; 
    fn2().await;
} 