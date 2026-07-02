use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use ureq::{Agent, AgentBuilder};
fn main() -> Result<(), ureq::Error> {
    ...
    let now = Instant::now();
    let agent = Arc::new(agent);
    let mut handles: Vec<thread::JoinHandle<Result<(), ureq::Error>>> = Vec::new();
    for web_page in webpages {
        let agent_thread = agent.clone();
        let t = thread::spawn(move || {
            let web_body = agent_thread.get(web_page).call()?.into_string()?;
            Ok(())
        });
        handles.push(t);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Time taken using Threads: {:.2?}", now.elapsed());
    Ok(())
}