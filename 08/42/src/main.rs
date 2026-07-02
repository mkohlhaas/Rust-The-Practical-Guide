impl Kmh {
  fn distance_in_three_hours(&self) -> Km {
    Km {
      value: self.value * 3,
    }
  }
}
