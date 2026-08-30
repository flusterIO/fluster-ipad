use crate::ecosystem::db::db_traits::db_field::{DatabaseField, DatabaseFieldLarge};

impl<T> DatabaseField for Option<T> where T: DatabaseField {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        T::field_definition(field_key, nullable)
    }
}

impl<T> DatabaseFieldLarge for Option<T> where T: DatabaseFieldLarge {
    fn field_definition_large(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        T::field_definition_large(field_key, nullable)
    }
}
