use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::ecosystem::db::db_traits::db_field::DatabaseField;

#[derive(strum_macros::Display, Serialize, Deserialize, Clone, Debug, Dummy, specta::Type)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ChatMessageSender {
    User,
    Agent,
    SystemPrompt,
}

impl DatabaseField for ChatMessageSender {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}
