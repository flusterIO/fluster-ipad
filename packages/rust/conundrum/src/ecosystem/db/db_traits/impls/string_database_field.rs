use std::sync::Arc;

use arrow_schema::Field;

use crate::ecosystem::db::db_traits::db_field::{DatabaseField, DatabaseFieldLarge};

impl DatabaseField for String {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key, arrow_schema::DataType::Utf8, nullable)
    }
}

impl DatabaseFieldLarge for String {
    fn field_definition_large(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key, arrow_schema::DataType::LargeUtf8, nullable)
    }
}

impl DatabaseField for Vec<String> {
    fn field_definition(field_key: &'static str, nullable: bool) -> Field {
        Field::new(field_key.to_string(),
                   arrow_schema::DataType::List(Arc::new(Field::new("item", arrow_schema::DataType::Utf8, true))),
                   nullable)
    }
}

impl DatabaseFieldLarge for Vec<String> {
    fn field_definition_large(field_key: &'static str, nullable: bool) -> Field {
        Field::new(field_key.to_string(),
                   arrow_schema::DataType::LargeList(Arc::new(Field::new("item", arrow_schema::DataType::Utf8, true))),
                   nullable)
    }
}
