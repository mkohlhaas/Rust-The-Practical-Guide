#[derive(Debug)]
enum List {
    Cons(i32, List), 
    Nil,
}  