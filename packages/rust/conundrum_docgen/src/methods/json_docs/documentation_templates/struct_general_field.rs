use rustdoc_types::Item;

#[derive(Clone, Debug)]
pub struct StructGeneralField<T: Clone> {
    pub item: Item,
    pub inner: T,
}
