use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
  for stream in listener.incoming() {
    // TcpStream implements traits `Read` and `Write`
    let stream: TcpStream = stream.unwrap();
    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&mut stream);
  let http_request = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|lines| !lines.is_empty())
    .collect::<Vec<String>>();

  println!("Request: {:#?}", http_request);
}
