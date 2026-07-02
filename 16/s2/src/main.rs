use std::{
  fs,
  io::{BufRead, BufReader, Write},
  net::{TcpListener, TcpStream},
  sync::{Arc, Mutex},
  thread,
  time::Duration,
};

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&mut stream);

  let request_lines: Vec<String> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

  let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nRequest Received";
  stream.write_all(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

fn main() {
  let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
  let active_requests = Arc::new(Mutex::new(0));
  let request_queue = Arc::new(Mutex::new(Vec::new()));

  for stream in listener.incoming() {
    let stream = stream.unwrap();
    let active_requests = Arc::clone(&active_requests);
    let request_queue = Arc::clone(&request_queue);

    thread::spawn(move || {
      {
        let mut connection = active_requests.lock().unwrap();
        *connection += 1;
        if *connection > 5 {
          let mut queue = request_queue.lock().unwrap();
          queue.push(stream);
          return;
        }
      }
      handle_connection(stream);
      {
        let mut connection = active_requests.lock().unwrap();
        *connection -= 1;
      }
      let mut queue = request_queue.lock().unwrap();
      if let Some(next_stream) = queue.pop() {
        handle_connection(next_stream);
      }
    });
  }
}
