impl IntoIterator for Book {
  type Item = String;
  type IntoIter = std::vec::IntoIter<Self::Item>;
  fn into_iter(self) -> Self::IntoIter {
    vec![self.title, self.author, self.genre].into_iter()
  }
}
