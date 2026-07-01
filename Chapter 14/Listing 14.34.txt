fn main() {
    let mut vec = vec![1, 2, 3];
    thread::scope(|some_scope| {
        some_scope.spawn(|| {
   println!("Thread inside scope);
            println!("{:?}", vec);
        });
    });

    println!("The scope finished");
    vec.push(5);
    println!("vec: {:?}", vec); 
}