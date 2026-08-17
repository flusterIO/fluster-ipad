use std::sync::Arc;

use conundrum::{
    ecosystem::db::db_traits::{db_entity::DBSchema, db_field::DatabaseField},
    lifted_models::primitives::db_id::DatabaseId,
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct AssignmentTag {
    pub tag_value: String,
    pub assignment_id: DatabaseId,
}
impl<'a> DBSchema<'a> for AssignmentTag {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(String::field_definition("tag_value", false)),
                Arc::new(DatabaseId::field_definition("assignment_id", false))])
    }
}
