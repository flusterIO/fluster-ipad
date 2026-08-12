use arrow_schema::{DataType, Field, Fields};

use crate::vector::database::db_traits::db_field::DatabaseField;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct PersonNameGroup {
    pub first: Option<String>,
    pub middle: Option<String>,
    pub last: Option<String>,
}

impl DatabaseField for PersonNameGroup {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key.to_string(),
                   DataType::Struct(Fields::from(vec![Field::new("first", DataType::Utf8, true),
                                                      Field::new("middle", DataType::Utf8, true),
                                                      Field::new("last", DataType::Utf8, true),])),
                   nullable)
    }
}
