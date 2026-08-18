use arrow_schema::{DataType, Field, Fields};
use conundrum::{ecosystem::db::db_traits::db_field::DatabaseField, lifted_models::primitives::date_time::DateTime};
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct AIGeneratedSummary {
    /// This is the conundrum content that you are providing to the user.
    /// Conundrum is an mdx like language with support for most of
    /// commonmark markdown, GFM, and a few additional syntaxes.
    /// Only use these additional syntaxes or Conundrum components if you are
    /// sure of the syntax. Summarizing the user's note well in markdown is
    /// more important than including advanced syntaxes or components.
    pub content: String,
    pub ctime: DateTime,
    /// This should be None if the item does not require an update, but will be
    /// set to the time the update _became_ required if it is indeed
    /// required. This allows for the debounce method to
    /// be used, significantly reducing token expedeture for content that will
    /// just be overwritten anyways.
    pub requires_update: Option<DateTime>,
}

impl DatabaseField for AIGeneratedSummary {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        Field::new(field_key.to_string(),
                   DataType::Struct(Fields::from(vec![String::field_definition("content", false),
                                                      DateTime::field_definition("ctime", false),
                                                      DateTime::field_definition("requires_update", false)])),
                   nullable)
    }
}
