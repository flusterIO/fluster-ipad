use arrow_schema::{DataType, Field, Fields};
use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use fake::Dummy;

use crate::vector::{
    database::db_traits::db_field::{DatabaseField, RepeatedDatabaseField},
    models::ai::{ai_generated_input::AIGeneratedInput, ai_notes::AINotes},
};

// # AI Interactions
//
// These fields describe the behavior AI should have with this specific instance
// of this model, and are your guide to this corner of your memory.
//
// The `notes` field is how the user can speak directly to you, and the
// `ai_generated_input` field is your way to keep track of everything having to
// do with this specific instance.
//
//  Make sure to keep track of all information necessary to provide a more
// in-depth experience for  the user in the `ai_generated_input` field, but
// **never** edit the `notes` field. Your goal is to grow your understanding of the user with each
// experience, and record your memory as markdown in the `ai_generated_input` field of each
// relevant model.
#[derive(serde::Serialize, serde::Deserialize, Default, Clone, Debug, specta::Type, Dummy)]
pub struct AIInteractions {
    pub notes: AINotes,
    pub ai_generated_input: AIGeneratedInput,
}

impl<'a> DBSchema<'a> for AIInteractions {}

impl DatabaseField for AIInteractions {
    fn field_definition(field_key: &'static str, nullable: bool) -> Field {
        Field::new(field_key.to_string(),
                   DataType::Struct(Fields::from(vec![Field::new("notes", DataType::Utf8, false),
                                                      Field::new("ai_generated_input", DataType::LargeUtf8, false)])),
                   nullable)
    }
}
