use arrow_schema::{DataType, Field, Fields};
use conundrum::ecosystem::db::db_traits::db_field::DatabaseField;
use fake::Dummy;

use crate::vector::models::ai::{ai_generated_input::AIGeneratedInput, ai_notes::AINotes};

/// # AI Interactions
/// Just like [AIInteractions](crate::vector::models::ai::ai_interactions::AIInteractions), but
/// partial
#[derive(serde::Serialize, serde::Deserialize, Default, Clone, Debug, specta::Type, Dummy)]
pub struct AIInteractionsPartial {
    pub notes: Option<AINotes>,
    pub ai_generated_input: Option<AIGeneratedInput>,
}

impl DatabaseField for AIInteractionsPartial {
    fn field_definition(field_key: &'static str, nullable: bool) -> Field {
        Field::new(field_key.to_string(),
                   DataType::Struct(Fields::from(vec![Field::new("notes", DataType::Utf8, false),
                                                      Field::new("ai_generated_input", DataType::LargeUtf8, false)])),
                   nullable)
    }
}
