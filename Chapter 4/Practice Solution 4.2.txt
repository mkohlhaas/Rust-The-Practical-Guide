fn main() {
let mut my_vec = vec![1, 2, 3, 4, 5];
    	let mut temp;

    	while !my_vec.is_empty() {
        temp = my_vec.clone(); // during the first iteration, the transfer of                 
                                  ownership occurs from my_vec to temp, which makes it   
                                  impossible to access the variable my_vec                  
                               in subsequent iterations
        println!("Elements in temporary vector are: {:?}", temp);
        if let Some(last_element) = my_vec.pop() { 
            println!("Popped element: {}", last_element);
        }
    }
}