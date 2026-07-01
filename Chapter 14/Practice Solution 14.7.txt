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
    let mut handles = vec![];
    let handle_1 = tokio::spawn(async move {
        fn1().await;
    });
    handles.push(handle_1);
    let handle_2 = tokio::spawn(async move {
        fn2().await;
    });
    handles.push(handle_2);
    for handle in handles {
        handle.await.unwrap(); 
    }
}