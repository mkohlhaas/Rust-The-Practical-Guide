use std::cell::RefCell;
fn main() {
    let data: RefCell<Option<i32>> = RefCell::new(Some(42));

    // TODO: Use borrow_mut to safely modify the value inside the RefCell to None.
    
    if /* TODO: add code to check if data contains the some variant*/ {
        println!("Final value: {:?}", data.borrow());
    } else {
        println!("No value present.");
    }
}