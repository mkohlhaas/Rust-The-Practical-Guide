fn main() { 
let x = 10;
    	stack_function(x);
	println!("In main, x is: {x}");
} 
fn stack_function(mut var: i32) {
    	var = 56;
    	println!("In func, var is: {var}");
} 