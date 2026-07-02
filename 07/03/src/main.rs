...
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_circle_should_contain_smaller() {
        let larger_circle = shapes::Circle::new(5.0);
        let smaller_circle = shapes::Circle::new(2.0);
        assert_eq!(larger_circle.contains(&smaller_circle), true);
    }
}
