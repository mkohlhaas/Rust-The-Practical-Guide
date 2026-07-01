fn main(){
    let x = match "123".parse::<i32>() {
        Ok(num) => num,
        Err(_) => panic!(),
    };
}