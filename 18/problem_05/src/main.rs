use std::collections::HashSet;

// customers can buy a maximum of two products using a gift card
fn product_suggestions(product_prices: Vec<i32>, amount: i32) -> Vec<Vec<i32>> {
  let mut prices = HashSet::new();
  let mut offers = Vec::new();

  // NOTE: Wrong! What if the customer buys only one product?
  for price in product_prices {
    let diff = amount - price;
    if prices.contains(&diff) {
      offers.push(vec![price, diff]);
    } else {
      prices.insert(price);
    }
  }

  offers
}

fn main() {
  // product prices
  let product = vec![11, 30, 55, 34, 45, 10, 19, 20, 60, 5, 23];

  let suggestions = product_suggestions(product, 50);

  println!("{:?}", suggestions);
}
