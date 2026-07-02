fn main() {
    let str_1 = String::from("abc");		// str_1 lifetime starts   
    str_fn(str_1); 					// str_1 lifetime ends 
    let str_2 = str_1; // Error
}

fn str_fn(s: String) {
    println!("s: {s}");
} 