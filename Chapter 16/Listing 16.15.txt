use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let mut active_requests = Arc::new(Mutex::new(0));
    for stream in listener.incoming() {
        let active_requests = Arc::clone(&active_requests);
        thread::spawn(move || {
            {
                let mut connection = active_requests.lock().unwrap();
                *connection += 1;
                if *connection >= 3 {
                    thread::sleep(Duration::from_secs(2));
                }
            }
            handle_connection(stream.unwrap());
            let mut connection = active_requests.lock().unwrap();
            *connection -= 1;
        });
    }
}