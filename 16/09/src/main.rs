use std::fs;
...
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|lines| !lines.is_empty())
        .collect::<Vec<String>>();
    println!("Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK \r\n";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();
    let responce = format!(
        "{} Contents-Length: {}\r\n\r\n{}",
        status_line, length, contents
    );
    stream.write_all(responce.as_bytes()).unwrap();
    stream.flush().unwrap();
}