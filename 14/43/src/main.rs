async fn printing() {
  println!("I am async function");
}

fn main() {
  let x = printing().await; // Error
}
