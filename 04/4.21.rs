fn main(){
	let mut vec_1 = vec![1, 2, 3]; 
	let ref1 = &vec_1; 		
	borrows_vec(ref1);  
	takes_and_gives_ownership(vec_1);
	println!("vec 1 is: {:?}", vec_1); // Error  
}
fn borrows_vec(vec: &Vec<i32>) {
    	println!("vec is: {:?}", vec);
}
fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
	vec.push(10); 
	vec
}