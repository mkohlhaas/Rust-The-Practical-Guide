trait Sound {
  fn animal_sound(&self) -> String; // Consider adding some code here
}
struct Dog;
struct Cat;
struct Fish;

impl Sound for Dog {
  fn animal_sound(&self) -> String {
    "woof".to_string()
  }
}
impl Sound for Cat {
  fn animal_sound(&self) -> String {
    "meow".to_string()
  }
}
impl Sound for Fish {} // fish does not make any sound so we should not implement the 
//fn animal_sound(). This will make compiler unhappy
fn main() {
  let dog = Dog;
  let cat = Cat;
  let fish = Fish;
  println!("Dog Sound: {}", dog.animal_sound());
  println!("Cat Sound: {}", cat.animal_sound());
  println!("Fish Sound: {}", fish.animal_sound());
}
