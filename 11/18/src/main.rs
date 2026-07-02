use std::rc::{self, Rc};
fn main() {
    let x = Rc::new(50);
    let y = x;
    println!("{}", x); // Error
}