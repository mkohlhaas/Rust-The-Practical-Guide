use std::thread;
fn main() {
    let mut vec = vec![1, 2, 3];
    let handle_1 = thread::spawn(|| { // Error
    println!("Thread 1");
        println!("{:?}", vec);
    });

    let handle_2 = thread::spawn(|| { // Error
        println!("Thread 2 ");
        println!("vec: {:?}", vec);
    });
    handle_1.join();
    handle_2.join();
    println!("The scope finished");
    vec.push(5); // Error
    println!("vec: {:?}", vec);
}