mod Order { 
    use crate::product::Product; 
    use crate::customer::Customer;
    struct Order {
        id: u64,
        product: Product,
        customer: Customer,
        quantity: u32,
    }
 ...
}
...