use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use serde_with::{DisplayFromStr, serde_as};
use strum::{EnumIter, IntoEnumIterator};

#[serde_as]
#[derive(serde::Serialize,
           serde::Deserialize,
           Clone,
           Debug,
           specta::Type,
           strum_macros::EnumString,
           strum_macros::EnumIter,
           strum_macros::Display)]
#[strum(serialize_all = "kebab-case")]
#[serde(try_from = "String")]
pub enum BodyPart {
    Calves,
    Quads,
    Hamstrings,
    Glutes,
    HipFlexors,
    HipAbductors,
    Abs,
    LowerBack,
    Lats,
    FrontDelts,
    SideDelts,
    RearDelts,
    Traps,
    Biceps,
    Triceps,
    Forearms,
    GripStrength,
}

impl TryFrom<String> for BodyPart {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for k in BodyPart::iter() {
            if k.to_string() == value {
                return Ok(k);
            }
        }
        return Err(DatabaseError::SerializationError);
    }
}
