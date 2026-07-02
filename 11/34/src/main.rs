fn main() {
    ...
    (*a).borrow_mut().next = Some(Rc::clone(&c));
    println!("\nAfter creating cycle:");
    println!("a strong count: {:?}", Rc::strong_count(&a));
    println!("b strong count: {:?}", Rc::strong_count(&b));
    println!("c strong count: {:?}", Rc::strong_count(&c));
}