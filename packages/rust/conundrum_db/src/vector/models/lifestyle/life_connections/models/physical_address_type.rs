use std::fmt::Display;

use conundrum::ecosystem::error_handling::db_error::DatabaseError;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub enum PhysicalAddressType {
    HouseOrApartment,
    /// An office or a factory. If the user works from home, the location type
    /// should then be 'personal-dwelling'.
    Workplace,
    Outdoors,
    /// The `group-venue` describes things like academic conferences and
    /// town-hall meetings.
    GroupVenue,
    /// A more specific event type. This cannot be more than 20 characters long.
    Other(String),
}

impl Display for PhysicalAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::HouseOrApartment => "personal-dwelling",
            Self::Workplace => "workplace",
            Self::Outdoors => "outdoors",
            Self::GroupVenue => "group-venue",
            Self::Other(s) => s.as_str(),
        })
    }
}

impl TryFrom<String> for PhysicalAddressType {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "personal-dwelling" => Ok(Self::HouseOrApartment),
            "workplace" => Ok(Self::Workplace),
            "outdoors" => Ok(Self::Outdoors),
            "group-venue" => Ok(Self::GroupVenue),
            _ => {
                if value.len() <= 20 {
                    Ok(Self::Other(value.to_lowercase()))
                } else {
                    log::error!("The PhysicalAddressType enum cannot be longer than 20 characters.");
                    Err(DatabaseError::SerializationError)
                }
            }
        }
    }
}
