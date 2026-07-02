use std::net::{TcpListener, TcpStream};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    for i in 0..10 {
        match listener.accept() {
            Ok((socket, addr)) => println!("The client info: {:?}", addr),
            Err(e) => println!("Couldn't get client: {:?}", e),
        }
    }
}