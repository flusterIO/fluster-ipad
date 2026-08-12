use std::sync::Arc;

use arrow_schema::Field;
use conundrum::ecosystem::db::traits::db_entity::DBSchema;

use crate::vector::{
    database::primitive_field_schema_generators::dual_id_fields::two_required_id_fields,
    models::primitives::db_id::DatabaseId,
};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, specta::Type, fake::Dummy)]
pub struct MilestoneAlarm {
    pub milestone_id: DatabaseId,
    pub alarm_id: DatabaseId,
}

impl<'a> DBSchema<'a> for MilestoneAlarm {
    fn arrow_fields() -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<Field>>> {
        let (f1, f2) = two_required_id_fields("milestone_id", "alarm_id");
        Ok(vec![Arc::new(f1), Arc::new(f2)])
    }
}
