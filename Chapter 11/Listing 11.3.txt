fn main() {
    ...
    let list_2 = Node {
        element: 1,
        next: Some(Box::new(Node {
            element: 2,
            next: Some(Box::new(Node {
                element: 3,
                next: None,
            })),
        })),
    };
} 