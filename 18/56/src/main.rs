impl DisplayLobby {
  fn new(root: Option<Rc<RefCell<Node>>>) -> Self {
    let mut stack = Vec::new();
    Self::push_all_left(root.clone(), &mut stack);
    DisplayLobby { stack }
  }
}
