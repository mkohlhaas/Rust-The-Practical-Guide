async fn printing(i: i32) {
    println!("Task {i}");
}
#[tokio::main(flavor = "current_thread")]
async fn main() {
    ...
}