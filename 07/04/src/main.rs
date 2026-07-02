...
#[cfg(test)]
mod tests {
    use super::*;
	...
    #[test]
    fn smaller_circle_should_not_contain_larger() {
        let larger_circle = shapes::Circle::new(5.0);
        let smaller_circle = shapes::Circle::new(2.0);
        assert_eq!(smaller_circle.contains(&larger_circle), false);
     }