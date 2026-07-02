fn main() {
    let mut emp_1 = Employee {
        name: String::from("John"),
        salary: 40_000,
    };
    let mut emp_2 = Employee {
        name: String::from("Joseph"),
        salary: 30_000,
    };
    let mut emp_db = Employee_Records {
        employee_db: vec![emp_1, emp_2],
    };
    println!("{:?}", emp_db.next());
    println!("{:?}", emp_db.next());
    println!("{:?}", emp_db.next());
}