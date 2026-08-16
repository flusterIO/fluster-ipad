use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use fake::Dummy;
use lancedb::query::ColumnOrdering;
use strum::IntoEnumIterator;

use crate::vector::database::db_traits::db_field::DatabaseField;

#[derive(serde::Serialize,
           serde::Deserialize,
           Clone,
           Debug,
           specta::Type,
           strum_macros::EnumIter,
           strum_macros::EnumString,
           strum_macros::Display,
           Dummy)]
#[strum(serialize_all = "kebab-case")]
#[serde(try_from = "String", into = "String", rename_all = "kebab-case")]
pub enum SortOrder {
    #[serde(rename = "asc-null-last")]
    #[strum(to_string = "asc-null-last")]
    AscNullLast,
    #[serde(rename = "desc-null-last")]
    #[strum(to_string = "desc-null-last")]
    DescNullLast,
    #[serde(rename = "asc-null-first")]
    #[strum(to_string = "asc-null-first")]
    AscNullFirst,
    #[serde(rename = "desc-null-first")]
    #[strum(to_string = "desc-null-first")]
    DescNullFirst,
}

impl Into<String> for SortOrder {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<String> for SortOrder {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for k in SortOrder::iter() {
            if k.to_string() == value {
                return Ok(k);
            }
        }
        return Err(DatabaseError::SerializationError);
    }
}

impl DatabaseField for SortOrder {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}

impl SortOrder {
    pub fn to_lancedb(&self, col: String) -> ColumnOrdering {
        match self {
            Self::AscNullLast => ColumnOrdering::asc_nulls_last(col),
            Self::AscNullFirst => ColumnOrdering::asc_nulls_first(col),
            Self::DescNullLast => ColumnOrdering::desc_nulls_last(col),
            Self::DescNullFirst => ColumnOrdering::desc_nulls_first(col),
        }
    }
}
