use std::hash::Hash;

use convert_case::Casing;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumIter};
use surrealdb::opt::Resource;
use surrealdb::types::SurrealValue;
use surrealdb_types::Table;

use crate::ecosystem::error_handling::db_error::DatabaseError;

#[derive(Debug,
           Serialize,
           Deserialize,
           Display,
           EnumIter,
           EnumCount,
           PartialEq,
           Clone,
           Eq,
           SurrealValue,
           specta::Type)]
#[surreal(untagged)]
pub enum DatabaseTable {
    // -- Pure Models --
    #[strum(to_string = "tag")]
    #[serde(rename = "tag")]
    #[surreal(value = "tag")]
    Tag,
    #[strum(to_string = "topic")]
    #[serde(rename = "topic")]
    #[surreal(value = "topic")]
    Topic,
    #[strum(to_string = "subject")]
    #[serde(rename = "subject")]
    #[surreal(value = "subject")]
    Subject,
    #[strum(to_string = "cdrm")]
    #[serde(rename = "cdrm")]
    #[surreal(value = "cdrm")]
    Cdrm,
    #[strum(to_string = "typst")]
    #[serde(rename = "typst")]
    #[surreal(value = "typst")]
    TypstContent,
    #[strum(to_string = "qa_pair")]
    #[serde(rename = "qa_pair")]
    #[surreal(value = "qa_pair")]
    QAPair,
    #[strum(to_string = "academic_res_metric")]
    #[serde(rename = "academic_res_metric")]
    #[surreal(value = "academic_res_metric")]
    AcademicResultMetric,
    #[strum(to_string = "bib_entry")]
    #[serde(rename = "bib_entry")]
    #[surreal(value = "bib_entry")]
    BibEntry,
    #[strum(to_string = "auto_taggable")]
    #[serde(rename = "auto_taggable")]
    #[surreal(value = "auto_taggable")]
    AutoTaggable,
    #[strum(to_string = "numeric_academic_res_metric")]
    #[serde(rename = "numeric_academic_res_metric")]
    #[surreal(value = "numeric_academic_res_metric")]
    /// Stores just the `AcademicResultMetricKey` and the value.
    NumericAcademicResultMetric,
    #[strum(to_string = "rational_academic_res_metric")]
    #[serde(rename = "rational_academic_res_metric")]
    #[surreal(value = "rational_academic_res_metric")]
    RationalScoreAcademicResultMetric,
    #[strum(to_string = "custom_academic_res_metric")]
    #[serde(rename = "custom_academic_res_metric")]
    #[surreal(value = "custom_academic_res_metric")]
    CustomAcademicResultMetric,
    /// ---- Vectors ----
    #[strum(to_string = "cdrm_vec")]
    #[serde(rename = "cdrm_vec")]
    #[surreal(value = "cdrm_vec")]
    CdrmVector,
    #[strum(to_string = "html_vec")]
    #[serde(rename = "html_vec")]
    #[surreal(value = "html_vec")]
    HtmlVector,
    #[strum(to_string = "typst_vec")]
    #[serde(rename = "typst_vec")]
    #[surreal(value = "typst_vec")]
    TypstVector,
}

impl Hash for DatabaseTable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl DatabaseTable {
    pub fn all_temporary_tables() -> Vec<Self> {
        vec![Self::CdrmVector]
    }

    pub fn is_schemafull(&self) -> bool {
        true
    }

    pub fn to_surreal_resource(&self) -> Resource {
        Resource::Table(Table::new(self.to_string()))
    }

    /// TODO: Move this to a macro or to a build-time calculation
    pub fn all_permanent_tables() -> Vec<Self> {
        let mut items = Vec::new();
        let temp_tables = Self::all_temporary_tables();
        for table in DatabaseTable::iter() {
            if !temp_tables.contains(&table) {
                items.push(table.clone());
            }
        }
        items
    }

    pub fn is_temporary_vector_table(&self) -> bool {
        match self {
            Self::CdrmVector => true,
            _ => false,
        }
    }

    /// Returns a name of the struct stored in the table for displaying user
    /// facing information.
    pub fn to_model_name(&self) -> String {
        match self {
            Self::Cdrm => String::from("Conundrum"),
            Self::QAPair => String::from("FlashCard"),
            _ => self.to_string().to_case(convert_case::Case::Title),
        }
    }
}

impl TryFrom<String> for DatabaseTable {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for s in Self::iter() {
            if s.to_string() == value {
                return Ok(s);
            }
        }
        return Err(DatabaseError::SerializationError);
    }
}
