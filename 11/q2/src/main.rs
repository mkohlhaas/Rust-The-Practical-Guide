#[derive(Debug)]
struct Linklist<?> {  // This line needs a fix 
    head: pointer<?>, // This line needs a fix 
}
#[derive(Debug)]
struct Node<?> {
    element: T,
    next: pointer<?>,  // This line needs a fix  
}
type pointer<?> = Option<Box<Node<?>>>;  // This line needs a fix 

impl<T: ?> Linklist<?> {  // This line needs a fix 
    fn new() -> Linklist<?> { // This line needs a fix 
        Linklist { head: None }
    }

    fn add(&mut self, element: i32) { // This line needs a fix 
        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element: element,
            next: previous_head,
        }));
        self.head = new_head;
    }

    fn remove(&mut self) -> Option<i32> {  // This line needs a fix 
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<i32> {  // This line needs a fix 
        match &self.head {
            Some(H) => Some(H.element),
            None => None,
        }
    }

    fn print(&self) {
        let mut list_traversal = &self.head;
        while !list_traversal.is_none() {
            println!("{:?}", list_traversal.as_ref().unwrap().element);
            list_traversal = &list_traversal.as_ref().unwrap().next;
        }
    }
}
fn main() {
    let mut list = Linklist::new();
    list.add(5);
    list.add(7);
    list.add(10);
    list.add(15);
    list.add(20);

    println!("{:?}", list.peek());
}