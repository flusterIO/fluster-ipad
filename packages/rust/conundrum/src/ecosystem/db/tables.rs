use std::hash::Hash;

use crate::ecosystem::error_handling::db_error::DatabaseError;
use convert_case::Casing;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumIter};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Deserialize, Display, EnumIter, EnumCount, PartialEq, Clone, Eq, specta::Type)]
pub enum DatabaseTable {
    // -- Pure Models --
    #[strum(to_string = "tag")]
    #[serde(rename = "tag")]
    Tag,
    #[strum(to_string = "topic")]
    #[serde(rename = "topic")]
    Topic,
    #[strum(to_string = "subject")]
    #[serde(rename = "subject")]
    Subject,
    #[strum(to_string = "cdrm")]
    #[serde(rename = "cdrm")]
    Cdrm,
    #[strum(to_string = "typst")]
    #[serde(rename = "typst")]
    TypstContent,
    #[strum(to_string = "user_workspace")]
    #[serde(rename = "user_workspace")]
    UserWorkspace,
    #[strum(to_string = "workspace_path")]
    #[serde(rename = "workspace_path")]
    WorkspacePath,
    #[strum(to_string = "qa_pair")]
    #[serde(rename = "qa_pair")]
    QAPair,
    #[strum(to_string = "academic_res_metric")]
    #[serde(rename = "academic_res_metric")]
    AcademicResultMetric,
    #[strum(to_string = "bib_entry")]
    #[serde(rename = "bib_entry")]
    BibEntry,
    #[strum(to_string = "auto_taggable")]
    #[serde(rename = "auto_taggable")]
    AutoTaggable,
    #[strum(to_string = "numeric_academic_res_metric")]
    #[serde(rename = "numeric_academic_res_metric")]
    /// Stores just the `AcademicResultMetricKey` and the value.
    NumericAcademicResultMetric,
    #[strum(to_string = "rational_academic_res_metric")]
    #[serde(rename = "rational_academic_res_metric")]
    RationalScoreAcademicResultMetric,
    #[strum(to_string = "custom_academic_res_metric")]
    #[serde(rename = "custom_academic_res_metric")]
    CustomAcademicResultMetric,
    #[strum(to_string = "git_repository")]
    #[serde(rename = "git_repository")]
    GitRepository,
    /// --- 'Joining' tables ---
    #[strum(to_string = "workspace_repository")]
    #[serde(rename = "workspace_repository")]
    WorkspaceRepository,
    /// ---- Vectors ----
    #[strum(to_string = "cdrm_vec")]
    #[serde(rename = "cdrm_vec")]
    MarkdownChunk,
}

impl Hash for DatabaseTable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl DatabaseTable {
    pub fn all_temporary_tables() -> Vec<Self> {
        vec![Self::MarkdownChunk]
    }

    /// Deprecated. Surreal was a hugeeee mistake.
    pub fn is_schemafull(&self) -> bool {
        true
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
            Self::MarkdownChunk => true,
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
