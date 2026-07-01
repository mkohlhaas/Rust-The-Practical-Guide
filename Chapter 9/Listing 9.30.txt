let result = words //Error
    .into_iter()
    .filter(|&word| word.starts_with("a") || word.starts_with("b"))
    .map(|word| word.to_uppercase())
    .collect(); 