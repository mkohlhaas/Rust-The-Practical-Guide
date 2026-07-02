use integration_tests::{Category, Customer, Order, Product};
mod helpers; 
#[test]
fn test_total_bill_without_discount() { 
	helpers::common_setup(); 
	...
}
...