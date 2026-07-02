#[derive(Debug)]
struct MRPProduct {
  map: HashMap<i32, Rc<RefCell<Node>>>,
  product_list: DoublyLinkList,
  size: i32,
  capacity: i32,
}
