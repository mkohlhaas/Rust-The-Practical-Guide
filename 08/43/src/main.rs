impl Mph {
  fn distance_in_three_hours(&self) -> Miles {
    Miles {
      value: self.value * 3,
    }
  }
}
