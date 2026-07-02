...
fn main() {
    ...
    let t = thread::spawn(|| {
    });
    println!("Hello 1 from the main");
    println!("Hello 2 from the main");
t.join();
   }
}