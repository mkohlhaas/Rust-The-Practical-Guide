- Singly linked list with Display trait
  - [100 Days of Rust — Day 16: Singly Linked List](https://medium.com/@shivamojha2419/100-days-of-rust-day-16-singly-linked-list-305461f282e9)
  - TODO: implement Display trait

- Rust is renowned for being a memory-safe language, offering strong guarantees
  against data races. However, Rust does not provide equally strict guarantees
  when it comes to memory leaks, that is, situations where memory is not properly
  freed up. While challenging in Rust, you can create memory that is never
  deallocated. Memory leaks can occur, particularly when using Rc and RefCell
  smart pointers, the latter of which enables mutability.
