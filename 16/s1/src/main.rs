use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
  println!("Server listening on 127.0.0.1:9000...");

  for stream in listener.incoming() {
    match stream {
      Ok(mut stream) => {
        handle_connection(stream);
      }
      Err(e) => {
        println!("Error accepting connection: {}", e);
      }
    }
  }
}

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&mut stream);

  let request_lines: Vec<String> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

  let first_line = &request_lines[0];
  let parts: Vec<&str> = first_line.split_whitespace().collect();
  let method = parts[0];
  let protocol = parts[2];
  println!("Method: {}", method);
  println!("Protocol: {}", protocol);

  let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nRequest received";
  stream.write(response.as_bytes()).unwrap();
}
