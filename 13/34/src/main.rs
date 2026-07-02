use std::marker::PhantomData;
struct ABC {
  ensuring_no_send_sync: PhantomData<Rc<()>>,
}
