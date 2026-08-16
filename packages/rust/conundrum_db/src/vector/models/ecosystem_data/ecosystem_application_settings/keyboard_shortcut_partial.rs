use std::sync::Arc;

use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_traits::db_field::DatabaseField,
    models::ecosystem_data::ecosystem_application_settings::ecosystem_application_action::EcosystemApplicationAction,
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct KeyboardShortcutPartial {
    pub action: EcosystemApplicationAction,
    pub key: Option<String>,
    /// The shift key was pressed.
    pub shift: Option<bool>,
    /// The meta key was pressed.
    pub meta: Option<bool>,
    /// The 'alt' ey was pressed.
    pub alt: Option<bool>,
    /// The 'option' key was pressed.
    pub ctrl: Option<bool>,
}

impl<'a> DBSchema<'a> for KeyboardShortcutPartial {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(EcosystemApplicationAction::field_definition("action", false)),
                Arc::new(String::field_definition("key", true)),
                Arc::new(bool::field_definition("shift", true)),
                Arc::new(bool::field_definition("meta", true)),
                Arc::new(bool::field_definition("alt", true)),
                Arc::new(bool::field_definition("ctrl", true)),])
    }
}
