fn products_in_range(root: Node, low: i32, high: i32) -> Vec<i32> {
  let mut output: Vec<i32> = Vec::new();
  traversal(Some(Box::new(root)), low, high, &mut output);
  output
}
