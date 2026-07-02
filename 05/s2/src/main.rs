#[derive(Debug)]
struct Item {
  id: i32,
  title: String,
  year: i32,
  type_: ItemType,
}

#[derive(Debug)]
enum ItemType {
  Book,
  Magazine,
}

fn display_item_info(item: &Item) {
  println!("Item ID: {:?}", item.id);
  println!("Title: {:?}", item.title);
  println!("Publication Year: {:?}", item.year);
  println!("Publication Type: {:?}", item.type_);
}

fn main() {
  let rust_book = Item {
    id: 1,
    title: String::from("The Rust Programming Language Book"),
    year: 2021,
    type_: ItemType::Book,
  };

  let rust_magazine = Item {
    id: 2,
    title: String::from("Rust Magazine"),
    year: 2022,
    type_: ItemType::Magazine,
  };

  display_item_info(&rust_book);
  display_item_info(&rust_magazine);
}
