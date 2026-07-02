let mut prod_without_none = Vec::new();
for p in products {
    if p.is_some() {
        prod_without_none.push(p.unwrap());
    }
}