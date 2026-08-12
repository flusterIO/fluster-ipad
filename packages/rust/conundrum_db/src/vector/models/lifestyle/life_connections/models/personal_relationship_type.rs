use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
#[serde(try_from = "String")]
pub enum PersonalRelationshipType {
    Friend,
    Colleague,
    Student,
    RomanticPartner,
    Teacher,
    Boss,
    Family,
    /// Note: The 'other' field must be fewer than 20 characters and should make
    /// sense in the phrase 'They are a <your input> to the user'.
    Other(String),
}

impl Display for PersonalRelationshipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Friend => "friend",
            Self::Colleague => "colleague",
            Self::Student => "student",
            Self::RomanticPartner => "romantic-partner",
            Self::Teacher => "teacher",
            Self::Boss => "boss",
            Self::Family => "family",
            Self::Other(s) => s,
        })
    }
}

impl TryFrom<String> for PersonalRelationshipType {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "colleague" => Ok(Self::Colleague),
            "friend" => Ok(Self::Friend),
            "student" => Ok(Self::Student),
            "teacher" => Ok(Self::Teacher),
            "boss" => Ok(Self::Boss),
            "romantic-partner" => Ok(Self::RomanticPartner),
            "family" => Ok(Self::Family),
            _ => {
                if value.len() <= 20 {
                    Ok(Self::Other(value.to_lowercase()))
                } else {
                    Err(DatabaseError::SerializationError)
                }
            }
        }
    }
}
