fn main(){
	let vec_1 = vec![1, 2, 3];
	let ref1 = &vec_1; 
	takes_ownership(ref1);  
	println!("vec 1 is: {:?}", vec_1); 
}
fn takes_ownership(vec: &Vec<i32>) {
    	println!("vec is: {:?}", vec);
}
