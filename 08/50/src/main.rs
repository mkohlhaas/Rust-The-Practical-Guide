trait Addition<Rhs> {
  type Output;
  fn add(self, rhs: Rhs) -> Self::Output;
}
