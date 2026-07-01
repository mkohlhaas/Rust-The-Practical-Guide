fn option(opt: Option<&i32>) -> &i32 {
    opt.unwrap()
}
fn main() {
    let y = 4; 
    let answer = { 
        option(Some(&y)) 
    };
    assert_eq!(answer, &4);
} 