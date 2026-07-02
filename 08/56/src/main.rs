fn main() {
  let p1 = Point { x: 1, y: 1 };
  let p2 = Point { x: 2, y: 2 };
  let p3 = p1.add(p2); // Error
  assert_eq!(p3.x, 3);
  assert_eq!(p3.y, 3);
}
