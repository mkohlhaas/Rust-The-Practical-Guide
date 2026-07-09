// Deriving Futures to Completion with the Await Method

// .await drives the futures to completion

async fn printing() {
  println!("I am async function");
}

fn main() {
  // x is unit type now
  let x: () = printing().await; // ⚠️ Error: main is not async
}
