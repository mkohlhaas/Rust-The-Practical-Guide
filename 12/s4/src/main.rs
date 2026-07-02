struct Books {
  books: u32,
}

struct Shelves {
  shelves: u32,
}

struct Library {
  books: Books,
  shelves: Shelves,
}

fn manage_books(b: &mut Books) -> &u32 {
  &b.books
}

fn calculate_shelf_space(s: &mut Shelves) -> u32 {
  s.shelves * 5
}

fn manage_library(lib: &mut Library) {
  let book_count = manage_books(&mut lib.books);
  let shelf_space = calculate_shelf_space(&mut lib.shelves);
  println!("{book_count}");
}

fn main() {
  let mut lib = Library {
    books: Books { books: 150 },
    shelves: Shelves { shelves: 20 },
  };

  manage_library(&mut lib);
}
