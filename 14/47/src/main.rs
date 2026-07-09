async fn printing() {
  println!("I am async function.");
}

#[tokio::main]
async fn main() {
  let x = printing();
  println!("The future has not been polled yet.");
  x.await; // futures are lazy
}
