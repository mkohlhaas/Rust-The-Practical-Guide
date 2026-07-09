#![allow(unused_variables)]

async fn printing() {
  println!("I am async function");
}

// ⚠️ Error: `main` function is not allowed to be `async`
async fn main() {
  let x = printing().await;
}
