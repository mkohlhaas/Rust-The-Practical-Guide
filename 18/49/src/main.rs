fn main() {
    let mut products_list = MRPProduct::new(3);
    products_list.purchase(10);
    products_list.print();
    products_list.purchase(15);
    products_list.print();
    products_list.purchase(20);
    products_list.print();
    products_list.purchase(25);
    products_list.print();
    products_list.purchase(20);
    products_list.print();
}