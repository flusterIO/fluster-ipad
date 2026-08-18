/// Some required methods for all structs capable of being used as a primary
/// key.
pub trait DatabaseIdentifiable {
    fn to_predicate(&self, field_key: &str) -> String;
}
