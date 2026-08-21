use arrow_schema::Field;
use conundrum::ecosystem::db::db_traits::db_field::{DatabaseField, DatabaseFieldLarge};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(transparent)]
pub struct Binary(Vec<u8>);

impl DatabaseField for Binary {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key.to_string(), arrow_schema::DataType::Binary, nullable)
    }
}

impl DatabaseFieldLarge for Binary {
    fn field_definition_large(field_key: &'static str, nullable: bool) -> Field {
        Field::new(field_key.to_string(), arrow_schema::DataType::LargeBinary, nullable)
    }
}
