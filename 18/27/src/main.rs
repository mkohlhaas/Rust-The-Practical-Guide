fn main() {
  let product_prices = vec![9, 6, 14, 20, 1, 30, 8, 17, 5];
  let mut bst = BinarySearchTree {
    root: Node::new(product_prices[0]),
  };

  for i in 1..product_prices.len() {
    bst.root.insert(product_prices[i]);
  }

  let result = products_in_range(bst.root, 6, 15);
  println!("{:?}", result);
}
