#[derive(Debug)]
enum ListNode<T> {
    /*TODO: Declare an enum variant called Node, with Box pointer for the next node of type 'T' */ 
    ///*TODO: Another variant for the placeholder for the end of the list
}

fn main() {
    // Create a linked list representing: Node(1, Node(2, Node(3, Node(4, None))))
    let list = ListNode::Node(1, /* TODO: Box pointer for the next node */);
    println!("{:?}", list);
}