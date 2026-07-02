fn main() {
  let mut list1 = Doubly_Linklist::new();
  list1.add(30);
  list1.add(32);
  list1.add(34);
  list1.add(36);
  list1.printing();
  list1.remove();
  println!("After Removal");
  list1.printing();
}
