#![allow(dead_code)]

pub use circle::Circle;
pub use drawing::DrawingInfo;
pub use rectangle::Rectangle;
pub use square::Square;
pub use traits::{Draw, Shape};

mod circle;
mod drawing;
mod rectangle;
mod square;
mod traits;
