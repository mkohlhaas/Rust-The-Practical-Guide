fn main() {
    let student_db = vec![
        …
    ];
    let student_name = String::from("Bob");
    let student_status = check_student(&student_name, &student_db); 

    match student_status {
            Ok(_) => {
		   Let student_grade = get_grade(&student_name, &student_db);	
                if let Some(grade) = student_grade {
                    println!("Grade is: {grade}");
                }
            }
            Err(error_msg) => println!("{error_msg}"),
        } 
}