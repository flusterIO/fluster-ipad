use chrono::Utc;
use fake::{Dummy, Fake, Faker, faker::chrono::en::DateTime as FakeChronoDateTime};
use lancedb::arrow::arrow_schema::Field;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::vector::database::db_traits::db_field::DatabaseField;

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct DateTime(chrono::DateTime<Utc>);

impl Dummy<Faker> for DateTime {
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let dt: chrono::DateTime<Utc> = FakeChronoDateTime().fake();
        Self(dt)
    }
}

impl DateTime {
    pub fn new_now() -> DateTime {
        let d = Utc::now();
        DateTime(d)
    }
}

impl DatabaseField<i64> for DateTime {
    fn field_definition(field_key: &'static str, nullable: bool) -> lancedb::arrow::arrow_schema::Field {
        Field::new(field_key, lancedb::arrow::arrow_schema::DataType::Timestamp(lancedb::arrow::arrow_schema::TimeUnit::Millisecond, Some("UTC".to_string().into()) ), nullable)
    }

    fn to_db_representation(&self) -> i64 {
        self.0.timestamp_millis()
    }
}
