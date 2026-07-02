struct Square { ... }
struct Rectangle { ... }
struct Circle { ... }

trait Draw { ... } 
trait Shape: Draw { ... } 
impl Shape for Rectangle { ... }
impl Shape for Square { ... } 
impl Draw for Rectangle { ... } // Added
impl Draw for Square { ... } // Added
fn shape_properties<T: Shape>(object: T){ ... }
fn returns_shape() -> impl Shape { ... }
fn main() {
    let r1 = Rectangle {
        ...
    };
    let s1 = Square {
        ...
    };
    shape_properties(r1);
    shape_properties(s1);
} 