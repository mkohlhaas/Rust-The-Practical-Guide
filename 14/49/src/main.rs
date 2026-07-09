// Tokio Tasks

const NUM_TASKS: u8 = 10;

async fn printing(i: u8) {
  println!("Task {:0>2}", i);
}

#[tokio::main]
async fn main() {
  let mut handles = vec![];

  for i in 1..=NUM_TASKS {
    // spawns a new asynchronous task, returning a `JoinHandle`
    let task = tokio::spawn(async move {
      println!("Task {:0>2}, printing, first time", i);
      printing(i).await;
      println!("Task {:0>2}, printing, second time", i);
      printing(i).await;
      println!("Task {:0>2}, completed", i);
    });
    handles.push(task);
  }

  for handle in handles {
    handle.await.unwrap();
  }

  println!("Done!");
}
