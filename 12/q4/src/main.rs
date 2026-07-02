struct Library {
  books: u32,
  shelves: u32,
  readers: u32,
}
fn manage_books(lib: &mut Library) -> &u32 {
  &lib.books
}
fn calculate_shelf_space(lib: &mut Library) -> u32 {
  lib.shelves * 5
}
fn manage_library(lib: &mut Library) {
  let book_count = manage_books(lib);
  let shelf_space = calculate_shelf_space(lib);
  println!("{book_count}");
}
