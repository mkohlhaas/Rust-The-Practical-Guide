struct Huge_Data;
fn main() {
  let data_1 = Huge_Data;
  let data_2 = Box::new(Huge_Data);
  let data_3 = data_1;
  let data_4 = data_2;
}
