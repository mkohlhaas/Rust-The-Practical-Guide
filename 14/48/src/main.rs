async fn printing() {
  println!("I am async function and nobody cares.");
}

#[tokio::main]
async fn main() {
  let x = printing();
  println!("The future has not been polled yet and never will.");

  drop(x); // cancel the future

  // x.await; // ⚠️ Error: use of moved value: `x`

  // x won't be polled now in the remaining code
  println!("Done!");
}
