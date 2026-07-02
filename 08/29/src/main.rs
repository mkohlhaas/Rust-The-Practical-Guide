struct Square { ... }
struct Rectangle { ... }
struct Circle { ... }

trait Draw {
    fn draw_object(&self);
}
trait Shape { ... }
impl Shape for Rectangle { ... }
impl Shape for Square { ... }
fn shape_properties<T: Shape>(object: T){ ... }
fn returns_shape() -> impl Shape { ... }
fn main() {  ... }