use std::fmt::Display;

use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use fake::Dummy;

/// The type of phone number.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
#[serde(into = "String", try_from = "String")]
pub enum PhoneNumberType {
    Home,
    Mobile,
    Work,
    Fax,
    Emergency,
    /// Cannot be more than 20 characters long.
    Other(String),
}

impl Display for PhoneNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Home => "home",
            Self::Mobile => "mobile",
            Self::Work => "work",
            Self::Fax => "fax",
            Self::Emergency => "emergecy",
            Self::Other(s) => s.as_str(),
        })
    }
}

impl Into<String> for PhoneNumberType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<String> for PhoneNumberType {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "home" => Ok(Self::Home),
            "mobile" => Ok(Self::Mobile),
            "work" => Ok(Self::Work),
            "fax" => Ok(Self::Fax),
            "emergency" => Ok(Self::Emergency),
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
