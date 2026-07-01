impl Student {
    pub fn new(std_name: String) -> Result<Self, String> {
        if std_name.chars().all(|x| matches!(x, 'a'..='z')) {
            Ok(Self {
                id: 0,
                age: 20,
                name: std_name,
            })
        } else {
            Err("The name is invalid.".to_string())
        }
    }
}