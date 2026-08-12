use arrow_schema::Field;
use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_traits::db_field::DatabaseField,
    models::{
        date_time::{alert::alert::Alert, date_time::DateTime},
        primitives::db_id::DatabaseId,
    },
};

#[derive(Debug, Deserialize, Serialize, Clone, specta::Type, Dummy)]
pub struct Alarm {
    pub id: DatabaseId,
    pub alert: Alert,
    pub time: DateTime,
}
