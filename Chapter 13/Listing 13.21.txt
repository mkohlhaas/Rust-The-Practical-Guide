trait Some_Trait {
    fn method(&self);
}
impl<T> Some_Trait for [T] {
    fn method(&self) {}
}