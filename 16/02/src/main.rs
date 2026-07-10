#![allow(unused_variables)]

use std::net::TcpListener;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
  for _ in 0..10 {
    // .accept() blocks
    match listener.accept() {
      Ok((socket, addr)) => println!("The client info: {:?}", addr),
      Err(e) => println!("Couldn't get client: {:?}", e),
    }
  }
}
