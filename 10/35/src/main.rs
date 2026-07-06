use std::cell::RefCell;

fn main() {
  // NOTE: `a` is not mutable
  let a = RefCell::new(10);

  // RefCell doesn't implement Deref to not violate borrowing rules
  // let c = *a; // ⚠️ type `RefCell<…>` cannot be dereferenced

  // interior mutability
  let mut ref1 = a.borrow_mut();
  *ref1 = 15;
  drop(ref1);

  println!("{:?}", a);
}
