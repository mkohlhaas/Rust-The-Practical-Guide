fn main() { 
    ...
    let list_3 = Linkedlist {
        head: Some(Node { // The head is wrapped by Some
            element: 1,
            next: Some(Box::new(Node {
                element: 2,
                next: Some(Box::new(Node {
                    element: 3,
                    next: None,
                })),
            })),
        }),
    };

    let list_4 = Linkedlist { head: None };  // This now compiles
} 