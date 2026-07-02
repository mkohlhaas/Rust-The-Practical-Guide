async fn printing() {
    println!("I am async function");
}

async fn main() { 
    let x = printing().await; 
}.await // Error