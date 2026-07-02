...
mod Order {
    struct Order {
        id: u64,
        product: crate::product::Product,   // Error
        customer: crate::customer::Customer, // Error
        quantity: u32,
    }
...