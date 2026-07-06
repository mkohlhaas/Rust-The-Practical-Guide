#![allow(dead_code, unused_variables)]

#[derive(Debug)]
struct HugeData;

#[derive(Debug)]
struct SmallData;

trait Storage {}

impl Storage for HugeData {}
impl Storage for SmallData {}

fn main() {
  let data1 = HugeData;
  let data2 = Box::new(HugeData);

  let data3: HugeData = data1;
  let data4: Box<HugeData> = data2;
  let data5: Box<SmallData> = Box::new(SmallData);

  // let data = vec![Box::new(data3), data4, data5]; // ⚠️ Error: mismatched types expected struct `Box<HugeData>` found struct `Box<SmallData>`

  let data: Vec<Box<dyn Storage>> = vec![Box::new(data3), data4, data5];

  // println!("{:?}", data); // ⚠️ Error: `dyn Storage` doesn't implement `Debug`
}
