impl DistanceThreeHours for Kmh {
  type Distance = Km;
  fn distance_in_three_hours(&self) -> Self::Distance {
    Self::Distance {
      value: self.value * 3,
    }
  }
}
