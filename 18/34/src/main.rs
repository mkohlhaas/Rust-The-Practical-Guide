use std::collections::HashMap;
#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct Node {
    children: HashMap<char, Node>,
    is_word: bool,
}