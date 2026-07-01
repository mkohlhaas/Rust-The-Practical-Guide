fn some_fn() {}
#[cfg(test)]
mod tests {
    ...
    #[test]
    fn larger_circle_should_contain_smaller() {
        some_fn();
	  ...
       }
}