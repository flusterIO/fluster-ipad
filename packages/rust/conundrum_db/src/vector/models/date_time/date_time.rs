use chrono::Utc;
use conundrum::ecosystem::db::traits::database_field_representable::DatabaseFieldRepresentable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DateTime(chrono::DateTime<Utc>);

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
