#![allow(dead_code, unused_variables)]

// `data reference` could become invalid during the execution of the program, while the instance of the struct is still alive.
// struct ArrayProcessor {
//   data: &[i32], // ⚠️ Error: missing lifetime specifier
// }

struct ArrayProcessor<'a> {
  data: &'a [i32],
}

// 1.
// impl<'a> ArrayProcessor<'a> {
//   fn update_data(&mut self, new_data: &[i32]) -> &[i32] {
//     let previous_data = self.data;
//     self.data = new_data; // ⚠️ Error
//     previous_data
//   }
// }

// 2.
// `new_data` has the same lifetime as its struct.
impl<'a> ArrayProcessor<'a> {
  fn update_data(&mut self, new_data: &'a [i32]) -> &[i32] {
    let previous_data = self.data;
    self.data = new_data;
    previous_data
  }
}

// 3.
// Without lifetime elision:
// impl<'a> ArrayProcessor<'a> {
//   fn update_data<'b>(&'b mut self, new_data: &'a [i32]) -> &'b [i32] {
//     let previous_data = self.data;
//     self.data = new_data;
//     previous_data
//   }
// }

// 4. with anonymous lifetimes
// impl<'a> ArrayProcessor<'a> {
//   fn update_data(&'_ mut self, new_data: &'a [i32]) -> &'_ [i32] {
//     let previous_data = self.data;
//     self.data = new_data;
//     previous_data
//   }
// }

// 5. ⚠️ Error
// impl<'a> ArrayProcessor<'a> {
//   fn update_data(&'a mut self, new_data: &'a [i32]) -> &'a [i32] {
//     let previous_data = self.data;
//     self.data = new_data;
//     previous_data
//   }
// }

fn main() {
  let mut some_data = ArrayProcessor { data: &[1, 2, 3] };
  let previous_data = some_data.update_data(&[4, 5, 6]);

  println!("Previous data: {:?}", previous_data);
  println!("New data: {:?}", some_data.data);
}
