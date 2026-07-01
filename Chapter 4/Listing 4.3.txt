fn main(){
let vec_1 = vec![1, 2, 3];
takes_ownership(vec_1);
println!("vec 1 is: {:?}", vec_1); // Error
}
fn takes_ownership(vec: Vec<i32>) {
    println!("vec is: {:?}", vec);
}