// Fixing module import and function usage

#![allow(dead_code)]

mod seasons {
  pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
  }

  pub fn is_holiday(season: &Season) -> bool {
    match season {
      Season::Summer => true,
      _ => false,
    }
  }
}

use seasons::{Season, is_holiday};

fn main() {
  let current_season = Season::Autumn;
  if is_holiday(&current_season) {
    println!("It's a holiday season! Time for a vacation!");
  } else {
    println!("Regular work season. Keep hustling!");
  }
}
