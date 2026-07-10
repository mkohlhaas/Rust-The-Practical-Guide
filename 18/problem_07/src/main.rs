use std::fmt::Debug;

type Pointer<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Linklist<T: Debug> {
  head: Pointer<T>,
}

#[derive(Debug)]
struct Node<T> {
  element: T,
  next: Pointer<T>,
}

impl<T: Debug> Linklist<T> {
  fn create_empty_list() -> Linklist<T> {
    Linklist { head: None }
  }

  fn add(&mut self, element: T) {
    let previous_head = self.head.take();

    let new_head = Box::new(Node {
      element,
      next: previous_head,
    });

    self.head = Some(new_head);
  }

  fn remove(&mut self) -> Option<T> {
    let head = self.head.take();

    match head {
      None => None,
      Some(head) => {
        self.head = head.next;
        Some(head.element)
      }
    }
  }

  #[allow(dead_code)]
  fn peek(&self) -> Option<&T> {
    match &self.head {
      Some(head) => Some(&head.element),
      None => None,
    }
  }

  // TODO: should be used in Display trait
  fn printing(&self) {
    let mut list_traversal = &self.head;
    println!();

    loop {
      match list_traversal {
        None => break,
        Some(node) => {
          print!("{:?} ", node.element);
          list_traversal = &list_traversal.as_ref().unwrap().next;
        }
      }
    }
  }

  // in-place reverse
  fn reverse(&mut self) {
    // empty list and list with one element are already reversed
    if self.head.is_none() || self.head.as_ref().unwrap().next.is_none() {
      return;
    }

    // init loop vars
    let mut previous = None;
    let mut current_node = self.head.take();

    while current_node.is_some() {
      // save next of current node (will be changed in the next step)
      let next = current_node.as_mut().unwrap().next.take();

      // point current to previous
      current_node.as_mut().unwrap().next = previous.take();

      // update loop vars
      previous = current_node.take();
      current_node = next;
    }

    self.head = previous.take();
  }
}

fn sorting_lists(lists: &mut Vec<Linklist<i32>>) -> Linklist<i32> {
  let mut sorted_list: Linklist<i32> = Linklist::create_empty_list();

  loop {
    let first_values = lists
      .iter_mut()
      .map(|list| list.head.as_ref().unwrap().element)
      .collect::<Vec<i32>>();

    println!("Values: {:?}", first_values);

    let min_val = *first_values.iter().min().unwrap();
    let min_list_index = first_values.iter().position(|n| *n == min_val).unwrap();

    sorted_list.add(min_val);
    lists[min_list_index].remove();

    // remove the whole list if it is empty
    if lists[min_list_index].head.is_none() {
      lists.remove(min_list_index);
    }

    if lists.is_empty() {
      break;
    }
  }

  sorted_list.reverse();
  sorted_list
}

fn main() {
  //list1, list2, list3 are already sorted

  let mut list1 = Linklist::create_empty_list();
  list1.add(45);
  list1.add(40);
  list1.add(35);
  list1.add(23);
  list1.add(11);

  let mut list2 = Linklist::create_empty_list();
  list2.add(60);
  list2.add(44);

  let mut list3 = Linklist::create_empty_list();
  list3.add(85);
  list3.add(20);
  list3.add(15);

  // list1.printing();
  // list2.printing();
  // list3.printing();

  let result = sorting_lists(&mut vec![list1, list2, list3]);
  result.printing();
}
