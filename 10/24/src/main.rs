#![allow(unused_variables)]

#[derive(Debug)]
struct HugeData;

fn main() {
  let data1 = HugeData;
  let data2 = Box::new(HugeData);

  // NOTE: When transferring ownership, data is copied around the stack!
  let data3 = data1; // expensive
  let data4 = data2; // cheap

  // println!("{:?}", data1); // ⚠️ Error
  // println!("{:?}", data2); // ⚠️ Error

  println!("{:?}", data3);
  println!("{:?}", data4);
}
