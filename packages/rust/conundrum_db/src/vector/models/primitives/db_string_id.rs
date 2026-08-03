use crate::vector::database::db_traits::db_identifiable::DatabaseIdentifiable;

impl DatabaseIdentifiable for String {
    fn to_predicate(&self, field_key: &str) -> String {
        format!("{} = {}", field_key, self)
    }
}
