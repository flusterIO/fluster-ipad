use std::sync::Arc;

use conundrum::{
    ecosystem::db::db_traits::{db_entity::DBSchema, db_field::DatabaseField},
    lifted_models::primitives::db_id::DatabaseId,
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct IDAndOptionalLabel {
    pub id: DatabaseId,
    pub label: Option<String>,
}

impl<'a> DBSchema<'a> for IDAndOptionalLabel {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)), Arc::new(String::field_definition("label", true))])
    }
}
