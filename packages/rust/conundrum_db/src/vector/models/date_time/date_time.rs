use chrono::Utc;
use conundrum::ecosystem::db::traits::database_field_representable::DatabaseFieldRepresentable;
use serde::{Deserialize, Serialize};

use crate::vector::database::db_traits::database_field::DatabaseField;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DateTime(chrono::DateTime<Utc>);

impl DatabaseField for DateTime {
    fn field_definition(field_key: &'static str, table: &conundrum::ecosystem::db::tables::DatabaseTable) -> String {
        format!("DEFINE FIELD {} ON {} TYPE datetime;", field_key, table)
    }
}

impl DateTime {
    pub fn new_now() -> DateTime {
        let d = Utc::now();
        DateTime(d)
    }
}

impl DatabaseFieldRepresentable<i64> for DateTime {
    fn to_db_representation(&self) -> i64 {
        self.0.timestamp_millis()
    }
}
