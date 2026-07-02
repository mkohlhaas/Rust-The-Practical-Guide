trait Addition {
  type Rhs;
  type Output;
  fn add(self, rhs: Self::Rhs) -> Self::Output;
}
