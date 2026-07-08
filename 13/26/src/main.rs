#![allow(dead_code, unused_variables)]

fn division_status(divident: f64, divisor: f64) -> Result<(), String> {
  let answer = match divisor {
    0.0 => Err(String::from("Error: Division by zero")),
    _ => Ok(()), // we don't care about the actual division
  };
  answer
}

fn main() {
  {
    let res1 = division_status(9.0, 3.0);
    let res2 = division_status(9.0, 0.0);

    println!("{:?}", res1);
    println!("{:?}", res2);
  }

  println!();

  {
    let mut vec: Vec<()> = Vec::with_capacity(0);

    vec.push(());
    vec.push(());
    vec.push(());

    println!("{}", vec.len()); // 3 

    // biggest possible value so the allocator is not pinged
    println!("{}", vec.capacity()); // 18446744073709551615 
  }
}
