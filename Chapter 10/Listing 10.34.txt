fn main() {
    ...
    let ref3 = x.borrow_mut();
    drop(ref3);
    println!("x: {:?}", x);
}