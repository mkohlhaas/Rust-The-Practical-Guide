fn main() {
  let mut my_vec = vec![1, 2, 3, 4, 5];
  // let mut temp;

  while !my_vec.is_empty() {
    // temp = my_vec; // Something wrong on this line
    // println!("Elements in temporary vector are: {:?}", temp);
    println!("Elements in temporary vector are: {:?}", my_vec);

    if let Some(last_element) = my_vec.pop() {
      println!("Popped element: {}", last_element);
    }
  }
}
