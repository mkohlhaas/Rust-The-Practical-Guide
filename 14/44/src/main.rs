async fn printing() {
    println!("I am async function");
}

async fn main() { // Error
    let x = printing().await;
}