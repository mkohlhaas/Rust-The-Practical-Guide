#[cfg(test)]
mod tests {
    ...
    fn larger_circle_should_contain_smaller() {
        ...
        assert_eq!(
            larger_circle.contains(&smaller_circle),
            true,
            "Custom failure message" // Custom message
        );
    }
    ...
}