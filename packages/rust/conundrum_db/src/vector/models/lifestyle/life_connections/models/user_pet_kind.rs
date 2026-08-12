use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use strum::IntoEnumIterator;

#[derive(serde::Serialize,
           serde::Deserialize,
           Clone,
           Debug,
           specta::Type,
           strum_macros::EnumIter,
           strum_macros::EnumString,
           strum_macros::Display)]
#[strum(serialize_all = "kebab-case")]
#[serde(try_from = "String")]
pub enum UserPetKind {
    Dog,
    Cat,
    Fish,
    Rabbit,
    GuineaPig,
    Snake,
    Lizard,
    Horse,
    /// The length of this 'other' field must not exceed 20 characters.
    Other(String),
}

impl TryFrom<String> for UserPetKind {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for k in UserPetKind::iter() {
            if k.to_string() == value {
                return Ok(k);
            }
        }
        if value.len() <= 20 {
            return Ok(Self::Other(value));
        } else {
            Err(DatabaseError::SerializationError)
        }
    }
}
