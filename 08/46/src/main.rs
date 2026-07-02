fn main() {
  let speed_Kmh = Kmh { value: 90 };
  let distance_Km = speed_Kmh.distance_in_three_hours();
  println!(
    "At {:?}, you’ll travel {:?} in 3 hours",
    speed_Kmh, distance_Km
  );

  let speed_Mph = Mph { value: 90 };
  let distance_Miles = speed_Mph.distance_in_three_hours();
  println!(
    "At {:?}, you’ll travel {:?}, in 3 hours",
    speed_Mph, distance_Miles
  );
}
