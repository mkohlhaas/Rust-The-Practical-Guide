use std::time::Duration;
use std::thread;
...
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut request_line = buf_reader.lines().next();
    let (status_line, file_name) = match request_line.unwrap().unwrap().as_str() {
        "GET / HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("index.html")),
        "GET /page1 HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10)); // added a delay
            (Some("HTTP/1.1 200 OK\r\n"), Some("page1.html"))
        }
        "GET /page2 HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("page2.html")),
        _ => (Some("HTTP/1.1 404 NOT FOUND\r\n"), Some("404.html")),
    };

    let contents = fs::read_to_string(file_name.unwrap()).unwrap();
    let responce = format!(
        "{} Content-Length: {}\r\n\r\n{}",
        status_line.unwrap(),
        contents.len(),
        contents
    );
    stream.write_all(responce.as_bytes()).unwrap();
    stream.flush().unwrap();
}