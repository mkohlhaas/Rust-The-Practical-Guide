#![allow(unused_variables)]

// Async Await

// Creating Async Functions

async fn printing() {
  println!("I am async function");
}

fn main() {
  // x: impl Future<Output = ()>
  // no output generated
  let x = printing(); // ⚠️ unused implementer of `Future` that must be used
}
