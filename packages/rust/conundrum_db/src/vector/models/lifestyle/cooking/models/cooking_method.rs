use std::fmt::Display;

use conundrum::ecosystem::error_handling::db_error::DatabaseError;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
#[serde(try_from = "String")]
pub enum CookingMethod {
    #[serde(rename = "baked")]
    Baked,
    #[serde(rename = "fried")]
    Fried,
    #[serde(rename = "boiled")]
    Boiled,
    #[serde(rename = "dried")]
    Dried,
    #[serde(rename = "raw")]
    Raw,
    /// Must be less than 20 characters in length.
    Other(String),
}

impl Display for CookingMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Baked => "baked",
            Self::Fried => "fried",
            Self::Boiled => "boiled",
            Self::Dried => "dried",
            Self::Raw => "raw",
            Self::Other(s) => s.as_str(),
        })
    }
}

impl TryFrom<String> for CookingMethod {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "baked" => Ok(Self::Baked),
            "fried" => Ok(Self::Fried),
            "boiled" => Ok(Self::Boiled),
            "dried" => Ok(Self::Dried),
            "raw" => Ok(Self::Raw),
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
