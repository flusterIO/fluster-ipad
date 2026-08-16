use arrow_schema::Field;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::database::db_traits::db_field::{DatabaseField, DatabaseFieldRepresentation};

/// A ridiculous id that always serializes to the same int so the same model is
/// always retrieved. Useful for things like the Settings struct, since the user
/// will only have one instance of it.
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct DBIDSingleInstance(i8);

impl DatabaseField for DBIDSingleInstance {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key.to_string(), arrow_schema::DataType::Int8, nullable)
    }
}

impl DatabaseFieldRepresentation<i8> for DBIDSingleInstance {
    fn to_db_representation(&self) -> i8 {
        self.0
    }
}
