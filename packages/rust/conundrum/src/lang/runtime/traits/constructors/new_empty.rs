pub trait EmptyConstructable {
    fn new_empty() -> Self
        where Self: Sized {
        todo!()
    }
}
