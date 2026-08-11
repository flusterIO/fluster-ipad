use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use fake::{Dummy, Faker};
use serde::{Deserialize, Serialize};

use crate::vector::database::db_traits::db_field::DatabaseField;

// ## AIGeneratedStatus
//
// This is the status that represents the amount of input AI had on the creation
// on a given instance of a database model. AI should always update this field
// accordingly when updating other parts of a model.
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
pub enum AIGeneratedStatus {
    /// This was completely written by humans.
    None = 0,
    /// This is primarily human written, but with some AI generated content.
    Some = 1,
    /// Mostly AI written with some human input
    Most = 2,
    /// Completely AI written.
    All = 3,
}

impl TryFrom<i64> for AIGeneratedStatus {
    type Error = DatabaseError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Some),
            2 => Ok(Self::Most),
            3 => Ok(Self::All),
            _ => Err(DatabaseError::FailToSerialize("Invalid AIGeneratedStatus value.".to_string())),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<i64> for AIGeneratedStatus {
    fn into(self) -> i64 {
        match self {
            Self::None => 0,
            Self::Some => 1,
            Self::Most => 2,
            Self::All => 3,
        }
    }
}

impl Dummy<Faker> for AIGeneratedStatus {
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        if rng.random_ratio(1, 4) {
            AIGeneratedStatus::None
        } else if rng.random_ratio(1, 4) {
            AIGeneratedStatus::Some
        } else if rng.random_ratio(1, 4) {
            AIGeneratedStatus::Most
        } else {
            AIGeneratedStatus::All
        }
    }
}

impl DatabaseField for AIGeneratedStatus {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}
