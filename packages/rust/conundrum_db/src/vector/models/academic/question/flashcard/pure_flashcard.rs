use conundrum::{
    ecosystem::db::tables::DatabaseTable, parsers::conundrum::logic::number::conundrum_float::ConundrumFloat,
};
use indoc::formatdoc;
use serde::Serialize;

use crate::vector::database::{
    db_traits::pure_model_static::PureModelStaticMethods,
    primitive_field_schema_generators::string_field_def_generator::{
        optional_float_field_definition, optional_string_field_definition, string_field_definition,
    },
};

#[derive(Clone, Debug, Serialize)]
pub struct PureFlashcard<T: Serialize> {
    pub question: String,
    /// Only valid type currently supported is a string, but this is in place to
    /// make room for numerical comparisons in the future.
    pub answer: T,
    /// ## AI
    /// This field is not optional for AI. Capable AI models shouold always
    /// provide an explanation.
    pub explanation: Option<String>,
    /// A number clampped between 0 and 100 indicating the difficulty of the
    /// question.
    ///
    /// ## AI
    ///
    /// AI should interpret this subjective scale with 100 being M.D. level
    /// biology or Ph.D. level physics, and 0 being elementary school
    /// mathematics like 2+2.
    pub difficulty: Option<ConundrumFloat>,
}

impl PureModelStaticMethods for PureFlashcard<String> {
    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::QAPair
    }

    fn schema() -> String {
        let tbl = Self::table();
        formatdoc! {"
        {}
        {}
        {}
        {}
            ", string_field_definition("question", &tbl), string_field_definition("answer", &tbl), optional_string_field_definition("explanation", &tbl), optional_float_field_definition("difficulty", &tbl)}
    }
}
