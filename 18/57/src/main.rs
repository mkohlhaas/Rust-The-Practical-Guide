fn main() {
  let mut bst = BinarySearchTree::new("Jeanette".to_string());
  let names: Vec<String> = vec![
    "Latasha",
    "Elvira",
    "Caryl",
    "Antoinette",
    "Cassie",
    "Charity",
    "Lyn",
    "Lia",
    "Anya",
    "Albert",
    "Cherlyn",
    "Lala",
    "Kandice",
    "Iliana",
    "Nouman",
    "Azam",
  ]
  .into_iter()
  .map(String::from)
  .collect();
  for name in names.into_iter() {
    bst.insert(name.to_string());
  }
  let mut display = DisplayLobby::new(Some(Rc::new(RefCell::new(bst.root))));
  println!("Participants on first page: {:?}", display.next_page());
  println!("Participants on second page: {:?}", display.next_page());
}
