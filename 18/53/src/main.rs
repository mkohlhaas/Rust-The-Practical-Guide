impl DisplayLobby {
    ...
    fn push_all_left(mut p: Option<Rc<RefCell<Node>>>, stack: &mut Vec<Rc<RefCell<Node>>>) {
        while let Some(link) = p.clone() {
            stack.push(p.clone().unwrap());
            p = link.borrow().left.clone();
        }
    }
}