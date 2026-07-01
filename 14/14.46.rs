async fn printing() {
    println!("I am async function");
}
#[tokio::main]
async fn main() {
    let x = printing().await;
}