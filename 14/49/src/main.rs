async fn printing(i: i32) {
    println!("Task {i}");
}
#[tokio::main]
async fn main() {
    let mut handles = vec![];
    for i in 0..3 {
        let handle = tokio::spawn(async move {
            println!("Task {i}, printing, first time");
            printing(i).await;
            println!("Task {i}, printing, second time");
            printing(i).await;
            println!("Task {i}, completed");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
    println!("All Tasks are now completed");
}