#[cfg(test)]
mod tests {
    ...
#[test]
#[should_panic]
fn should_not_create_and_panic() {
    	    let some_circle = shapes::Circle::new_2(-11.0); 
}
}