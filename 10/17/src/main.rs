impl<'a> ArrayProcessor<'a> {
  fn update_data(&mut self, new_data: &[i32]) -> &[i32] {
    let previous_data = self.data;
    self.data = new_data; // Error
    previous_data
  }
}
