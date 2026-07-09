#![allow(unused_variables)]

// Web Scraping Using Threads

use std::time::Instant;
use ureq::AgentBuilder;

fn main() -> Result<(), ureq::Error> {
  let webpages = vec![
    "https://gist.github.com/recluze/1d2989c7e345c8c3c542",
    "https://gist.github.com/recluze/a98aa1804884ca3b3ad3",
    "https://gist.github.com/recluze/5051735efe3fc189b90d",
    "https://gist.github.com/recluze/460157afc6a7492555bb",
    "https://gist.github.com/recluze/5051735efe3fc189b90d",
    "https://gist.github.com/recluze/c9bc4130af995c36176d",
    "https://gist.github.com/recluze/1d2989c7e345c8c3c542",
    "https://gist.github.com/recluze/a98aa1804884ca3b3ad3",
    "https://gist.github.com/recluze/5051735efe3fc189b90d",
    "https://gist.github.com/recluze/460157afc6a7492555bb",
    "https://gist.github.com/recluze/5051735efe3fc189b90d",
    "https://gist.github.com/recluze/c9bc4130af995c36176d",
    "https://gist.github.com/recluze/1d2989c7e345c8c3c542",
    "https://gist.github.com/recluze/a98aa1804884ca3b3ad3",
    "https://gist.github.com/recluze/5051735efe3fc189b90d",
    "https://gist.github.com/recluze/460157afc6a7492555bb",
    "https://gist.github.com/recluze/5051735efe3fc189b90d",
    "https://gist.github.com/recluze/c9bc4130af995c36176d",
  ];

  let agent: ureq::Agent = AgentBuilder::new().build();
  let now = Instant::now();

  for web_page in &webpages {
    let web_body = agent.get(web_page).call()?.into_string()?;
    // println!("{}", web_body);
  }

  println!("Time taken without threads: {:.2?}", now.elapsed());
  Ok(())
}
