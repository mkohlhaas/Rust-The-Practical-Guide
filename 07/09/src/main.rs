#[cfg(test)]
mod tests {
    ...
        #[test]
    	  fn should_not_create_circle() -> Result<(), String> {
            let some_circle = shapes::Circle::new_1(-1.0)?;
            Ok(())
    }
}