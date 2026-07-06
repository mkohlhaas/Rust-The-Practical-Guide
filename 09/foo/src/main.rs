#![allow(dead_code, unused_variables)]

use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct User {
  name: String,
  age: u8,
  salary: u32,
}

impl User {
  fn new(name: String, age: u8, salary: u32) -> Self {
    Self { name, age, salary }
  }
}

fn validate_user_fn(user: &User) -> bool {
  !user.name.is_empty()
}

#[derive(Debug)]
struct Employee {
  name: String,
  salary: u16,
}

impl Employee {
  fn new(name: String, salary: u16) -> Self {
    Self { name, salary }
  }
}

#[derive(Debug)]
struct EmployeeRecords {
  employee_db: VecDeque<Employee>,
}

impl EmployeeRecords {
  fn new(employee_db: VecDeque<Employee>) -> Self {
    Self { employee_db }
  }
}

impl From<VecDeque<Employee>> for EmployeeRecords {
  fn from(employee_db: VecDeque<Employee>) -> Self {
    Self { employee_db }
  }
}

impl Iterator for EmployeeRecords {
  type Item = Employee;

  fn next(&mut self) -> Option<Self::Item> {
    self.employee_db.pop_front()
  }
}

// impl Iterator for EmployeeRecords {
//   type Item = String;
//
//   fn next(&mut self) -> Option<Self::Item> {
//     if self.employee_db.is_empty() {
//       None
//     } else {
//       let result = self.employee_db[0].name.clone();
//       self.employee_db.remove(0);
//       Some(result)
//     }
//   }
// }

// https://doc.rust-lang.org/nightly/core/iter/index.html#implementing-iterator
// There are three common methods which can create iterators from a collection:
//   - iter()      which iterates over &T
//   - iter_mut()  which iterates over &mut T
//   - into_iter() which iterates over T
//
// `for` loops use IntoIterator
struct Book {
  title: String,
  author: String,
  genre: String,
}

struct BookIterator {
  properties: Vec<String>,
}

impl Iterator for BookIterator {
  type Item = String;
  fn next(&mut self) -> Option<Self::Item> {
    if !self.properties.is_empty() {
      Some(self.properties.remove(0))
    } else {
      None
    }
  }
}

impl IntoIterator for Book {
  type Item = String;
  type IntoIter = BookIterator;

  fn into_iter(self) -> Self::IntoIter {
    Self::IntoIter {
      properties: vec![self.title, self.author, self.genre],
    }
  }
}

// impl IntoIterator for Book {
//   type Item = String;
//   type IntoIter = std::vec::IntoIter<Self::Item>;
//
//   fn into_iter(self) -> Self::IntoIter {
//     vec![self.title, self.author, self.genre].into_iter()
//   }
// }

fn main() {
  // //////// //
  // Closures //
  // //////// //
  {
    println!("------------");

    let u1 = User::new(String::from("John"), 35, 42000);
    let u2 = User::new(String::from(""), 35, 42000);

    println!("{:?}", u1);
    println!("{:?}", u2);

    println!("------------");

    println!("{}", validate_user_fn(&u1));
    println!("{}", validate_user_fn(&u2));
  }

  println!("------------");

  {
    let validate_user_clos = |user: &User| !user.name.is_empty();

    let u1 = User::new(String::from("John"), 35, 42000);
    let u2 = User::new(String::from(""), 35, 42000);

    println!("{}", validate_user_clos(&u1));
    println!("{}", validate_user_clos(&u2));
  }

  println!("------------");

  {
    let validate_user_simple = |user: &User| !user.name.is_empty();
    let validate_user_advance = |user: &User| user.age >= 30;

    fn is_valid_user<V1, V2>(user: &User, simple_validator: V1, advance_validator: V2) -> bool
    where
      V1: Fn(&User) -> bool,
      V2: Fn(&User) -> bool,
    {
      simple_validator(user) && advance_validator(user)
    }

    let u1 = User::new(String::from("John"), 35, 42000);
    let u2 = User::new(String::from(""), 35, 42000);

    println!(
      "{}",
      is_valid_user(&u1, validate_user_simple, validate_user_advance)
    );

    println!(
      "{}",
      is_valid_user(&u2, validate_user_simple, validate_user_advance)
    );
  }

  println!("------------");

  {
    let /* mut */ banned_user = String::from("banned user");

    let /* mut */ validate_user_simple = /* move */ |user: &User| {
      // let banned_user_name = &mut banned_user;
      !user.name.is_empty() && banned_user != user.name
    };

    let validate_user_advance = |user: &User| user.age >= 30;

    fn is_valid_user<V1, V2>(user: &User, simple_validator: V1, advance_validator: V2) -> bool
    where
      V1: Fn(&User) -> bool,
      V2: Fn(&User) -> bool,
    {
      simple_validator(user) && advance_validator(user)
    }

    let u1 = User::new(String::from("John"), 35, 42000);
    let u2 = User::new(String::from("banned user"), 35, 42000);

    println!(
      "{}",
      is_valid_user(&u1, &validate_user_simple, &validate_user_advance)
    );

    println!(
      "{}",
      is_valid_user(&u2, &validate_user_simple, &validate_user_advance)
    );

    println!("{}", banned_user);
  }

  println!("------------");

  // ///////////////// //
  // Function Pointers //
  // ///////////////// //

  {
    // NOTE: Function pointers implement all the three closure traits, that is, Fn, FnMut, and FnOnce.
    //       In other words, you can pass in regular functions anywhere closures are expected.
    fn validate_user_simple(user: &User) -> bool {
      !user.name.is_empty()
    }

    fn validate_user_advance(user: &User) -> bool {
      user.age >= 30
    }

    fn is_valid_user<V1, V2>(user: &User, simple_validator: V1, advance_validator: V2) -> bool
    where
      V1: Fn(&User) -> bool,
      V2: Fn(&User) -> bool,
    {
      simple_validator(user) && advance_validator(user)
    }

    let u1 = User::new(String::from("John"), 35, 42000);
    let u2 = User::new(String::from(""), 35, 42000);

    println!(
      "{}",
      is_valid_user(&u1, validate_user_simple, validate_user_advance)
    );

    println!(
      "{}",
      is_valid_user(&u2, validate_user_simple, validate_user_advance)
    );
  }

  println!("------------");

  {
    // NOTE: Function pointers implement all the three closure traits, that is, Fn, FnMut, and FnOnce.
    //       In other words, you can pass in regular functions anywhere closures are expected.
    fn validate_user_simple(user: &User) -> bool {
      !user.name.is_empty()
    }

    fn validate_user_advance(user: &User) -> bool {
      user.age >= 30
    }

    fn is_valid_user(
      user: &User,
      simple_validator: fn(&User) -> bool,
      advance_validator: fn(&User) -> bool,
    ) -> bool {
      simple_validator(user) && advance_validator(user)
    }

    let u1 = User::new(String::from("John"), 35, 42000);
    let u2 = User::new(String::from(""), 35, 42000);

    println!(
      "{}",
      is_valid_user(&u1, validate_user_simple, validate_user_advance)
    );

    println!(
      "{}",
      is_valid_user(&u2, validate_user_simple, validate_user_advance)
    );
  }

  println!("------------");

  {
    let banned_user = String::from("banned user");

    fn validate_user_simple(user: &User, banned_user_name: &str) -> bool {
      !user.name.is_empty() && banned_user_name != user.name
    }

    fn validate_user_advance(user: &User) -> bool {
      user.age >= 30
    }

    fn is_valid_user(
      user: &User,
      banned_user: &str,
      simple_validator: fn(&User, &str) -> bool,
      advance_validator: fn(&User) -> bool,
    ) -> bool {
      simple_validator(user, banned_user) && advance_validator(user)
    }

    let u1 = User::new(String::from("John"), 35, 42000);
    let u2 = User::new(String::from("banned user"), 35, 42000);

    println!(
      "{}",
      is_valid_user(
        &u1,
        &banned_user,
        validate_user_simple,
        validate_user_advance
      )
    );

    println!(
      "{}",
      is_valid_user(
        &u2,
        &banned_user,
        validate_user_simple,
        validate_user_advance
      )
    );

    println!("{}", banned_user);
  }

  println!("------------");

  // ///////// //
  // Iterators //
  // ///////// //

  {
    let emp1 = Employee::new(String::from("John"), 40_000);
    let emp2 = Employee::new(String::from("Joseph"), 30_000);

    let mut emp_db = EmployeeRecords::new(VecDeque::from(vec![emp1, emp2]));

    println!("{:?}", emp_db.next());
    println!("{:?}", emp_db.next());
    println!("{:?}", emp_db.next());
    println!("{:?}", emp_db.next());
    println!("{:?}", emp_db.next());
  }

  println!("------------");

  {
    let emp1 = Employee::new(String::from("John"), 40_000);
    let emp2 = Employee::new(String::from("Joseph"), 30_000);

    let mut emp_db = EmployeeRecords::new(VecDeque::from(vec![emp1, emp2]));

    for employee in &mut emp_db {
      println!("{:?}", employee);
    }

    // NOTE: empty!!!
    println!("{:?}", emp_db);
  }

  println!("------------");

  {
    let book = Book {
      title: "Digital Image Processing".to_string(),
      author: "Gonzales".to_string(),
      genre: "Science Book".to_string(),
    };

    let mut book_iterator = book.into_iter();
    println!("{:?}", book_iterator.next());
    println!("{:?}", book_iterator.next());
    println!("{:?}", book_iterator.next());
    println!("{:?}", book_iterator.next());
  }

  println!("------------");

  {
    let book = Book {
      title: "Digital Image Processing".to_string(),
      author: "Gonzales".to_string(),
      genre: "Science Book".to_string(),
    };

    let book_iterator = book.into_iter();

    for book_info in book_iterator {
      println!("{book_info}");
    }
  }

  println!("------------");

  {
    let book = Book {
      title: "Digital Image Processing".to_string(),
      author: "Gonzales".to_string(),
      genre: "Science Book".to_string(),
    };

    for book_info in book {
      println!("{book_info}");
    }
  }

  println!("------------");

  // 1.  iter()      which iterates over &T     (borrows immutably)
  // 2.  iter_mut()  which iterates over &mut T (borrows mutably)
  // 3.  into_iter() which iterates over T      (consumes T)

  {
    // 1.
    let vec1 = vec![45, 30, 85, 90, 41, 39];

    // uses iter()
    for item in &vec1 {
      println!("{}", item);
    }

    println!("{:?}", vec1);
  }

  println!("------------");

  {
    // 2.
    let mut vec1 = vec![45, 30, 85, 90, 41, 39];

    // uses iter_mut()
    for item in &mut vec1 {
      println!("{}", item);
    }

    println!("{:?}", vec1);
  }

  println!("------------");

  {
    // 3.
    let vec1 = vec![45, 30, 85, 90, 41, 39];

    // uses into_iter()
    for item in vec1 {
      println!("{}", item);
    }

    // println!("{:?}", vec1); // ⚠️ error
  }

  println!("------------");

  {
    let mut persons: HashMap<String, i32> = HashMap::new();

    persons.insert("Hannash".to_string(), 40);
    persons.insert("Joseph".to_string(), 44);
    persons.insert("Sara".to_string(), 55);

    for (name, age) in &mut persons {
      println!("Name: {}, Age: {}", name, age);
    }

    println!("{:?}", persons);
  }

  // /////////// //
  // Combinators //
  // /////////// //

  // Combinators are methods provided by the Iterator trait that allow us
  // to transform, filter, or combine elements in a chainable manner.

  println!("------------");

  {
    let words = vec!["apple", "banana", "grape", "orange", "pear"];
    println!("{:?}", words);

    let mut result: Vec<String> = vec![];

    for word in words {
      if word.starts_with("a") || word.starts_with("b") {
        let uppercase_word = word.to_uppercase();
        result.push(uppercase_word);
      }
    }
    println!("{:?}", result);
    // println!("{:?}", words); // ⚠️ error
  }

  println!("------------");

  {
    let mut words = vec!["apple", "banana", "grape", "orange", "pear"];
    println!("{:?}", words);

    let result = words
      .iter_mut()
      .filter(|word| word.starts_with("a") || word.starts_with("b"))
      .map(|word| word.to_uppercase())
      .collect::<Vec<String>>();

    println!("{:?}", result);
    println!("{:?}", words);
  }

  println!("------------");

  {
    let words = vec!["apple", "banana", "grape", "orange", "pear"];
    println!("{:?}", words);

    let result = words
      .into_iter()
      .filter(|word| word.starts_with("a") || word.starts_with("b"))
      .map(|word| word.to_uppercase())
      .collect::<Vec<String>>();

    println!("{:?}", result);
    // println!("{:?}", words); // ⚠️ error
  }

  println!("------------");

  {
    let words = vec!["apple", "banana", "grape", "orange", "pear"];
    println!("{:?}", words);

    let result = words
      .iter()
      .filter(|word| word.starts_with("a") || word.starts_with("b"))
      .map(|word| word.to_uppercase())
      .collect::<Vec<String>>();

    println!("{:?}", result);
    println!("{:?}", words);
  }

  // //////////////////////// //
  // Iterating through Option //
  // //////////////////////// //

  println!("------------");

  {
    let some_product = Some("laptop");
    let mut products = vec!["cellphone", "battery", "charger"];

    match some_product {
      Some(product) => products.push(product),
      _ => {}
    };
    println!("{:?}", products);
  }

  println!("------------");

  {
    let some_product = Some("laptop");
    let mut products = vec!["cellphone", "battery", "charger"];

    if let Some(product) = some_product {
      products.push(product)
    };
    println!("{:?}", products);
  }

  println!("------------");

  {
    let some_product = Some("laptop");
    let mut products = vec!["cellphone", "battery", "charger"];

    // Option implements IntoIterator
    products.extend(some_product);
    println!("{:?}", products);

    let mut message = String::from("abc");
    message.extend(['d', 'e', 'f']);
    println!("{}", message);
  }

  println!("------------");

  {
    let some_product = Some("laptop");
    let products = vec!["cellphone", "battery", "charger"];
    let products_iter = products.iter().chain(&some_product);
    for prod in products_iter {
      println!("{} ", prod);
    }

    let s1 = "abc".chars();
    let s2 = "def".chars();
    let iter = s1.chain(s2);
    for c in iter {
      println!("{}", c);
    }
  }

  println!("------------");

  {
    let products = vec![Some("charger"), Some("battery"), None, Some("cellphone")];

    let mut prod_without_none = Vec::new();
    for p in products {
      if p.is_some() {
        prod_without_none.push(p.unwrap());
      }
    }
    println!("{:?}", prod_without_none);
  }

  println!("------------");

  {
    let products = vec![Some("charger"), Some("battery"), None, Some("cellphone")];
    let prod_without_none = products
      .into_iter()
      .filter(|x| x.is_some())
      .map(|x| x.unwrap())
      .collect::<Vec<&str>>();
    println!("{:?}", prod_without_none);
  }

  println!("------------");

  {
    // https://www.40tude.fr/docs/06_programmation/rust/020_some/some_02.html

    let products = vec![Some("charger"), Some("battery"), None, Some("cellphone")];
    let prod_without_none: Vec<&str> = products.into_iter().flatten().collect();
    println!("{:?}", prod_without_none);
  }
}
