use std::sync::Arc;

use arrow_schema::Field;
use conundrum::{
    ecosystem::db::{db_traits::db_entity::DBSchema, tables::DatabaseTable},
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_macros::DatabaseEntity;

use crate::vector::database::primitive_field_schema_generators::dual_id_fields::two_required_id_fields;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, specta::Type, fake::Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::MilestoneAlarm)]
pub struct MilestoneAlarm {
    pub id: DatabaseId,
    pub milestone_id: DatabaseId,
    pub alarm_id: DatabaseId,
}
