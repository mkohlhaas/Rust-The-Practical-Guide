use std::thread;
fn main() {
    let mut vec = vec![1, 2, 3];
    thread::scope(|some_scope| {
        ...
        some_scope.spawn(|| {
            println!("Another Thread inside scope");
            // vec.push(4); // updated line
            println!("vec: {:?}", vec);
        });
    });
...
}