fn main() { 
    ...
     let list_3 = Linkedlist {
        head: Node {
            element: 1,
            next: Some(Box::new(Node {
                element: 2,
                next: Some(Box::new(Node {
                    element: 3,
                    next: None,
                })),
            })),
        },
    };
}