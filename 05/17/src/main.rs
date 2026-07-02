impl TravelType {
  fn travel_allowance(&self, miles: f32) -> f32 {
    let allowance = match self {
      TravelType::Car => miles * 2.0,
      TravelType::Train => miles * 3.0,
      TravelType::Airplane => miles * 5.0,
    };
    allowance
  }
}
