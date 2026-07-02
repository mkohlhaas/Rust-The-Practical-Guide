trait Drawable {
  fn draw(&self);
}
trait AnimatedDrawable: Drawable {
  fn animate(&self);
}
struct Circle;
/* some code missing here */
impl AnimatedDrawable for Circle {
  fn animate(&self) {
    println!("Animating a circle");
  }
}

fn main() {
  let circle = Circle;
  circle.draw();
  circle.animate();
}
