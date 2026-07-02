fn main() {
  let mut some_data = ArrayProcessor { data: &[4, 5, 6] };
  let previous_data = some_data.update_data(&[5, 8, 10]);
  println!("Previous data: {:?}", previous_data);
  println!("New data: {:?}", some_data.data);
}
