...
fn main() {
    ...
    let t = thread::spawn(|| {
    });
t.join();
    println!("Hello 1 from the main");
    println!("Hello 2 from the main");
   }
}