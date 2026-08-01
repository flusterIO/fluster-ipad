use chrono::Utc;
use fake::{faker::chrono::en::DateTime as FakeChronoDateTime, Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};
use specta::Type;

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
