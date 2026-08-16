use std::sync::Arc;

use conundrum::ecosystem::db::traits::db_entity::{DBEntity, DBSchema};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::{
    impl_default_crud,
    vector::{
        database::db_traits::db_field::DatabaseField,
        models::ecosystem_data::ecosystem_application_settings::{
            ecosystem_application_action::EcosystemApplicationAction,
            keyboard_shortcut_partial::KeyboardShortcutPartial,
        },
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct KeyboardShortcut {
    pub action: EcosystemApplicationAction,
    pub key: String,
    /// The meta key was pressed.
    pub shift: bool,
    /// The meta key was pressed.
    pub meta: bool,
    /// The 'alt' ey was pressed.
    pub alt: bool,
    /// The 'crl' key was pressed.
    pub ctrl: bool,
}

impl_default_crud!(KeyboardShortcut, KeyboardShortcutPartial, String);

impl<'a> DBEntity<'a> for KeyboardShortcut {
    type PartialUpdateType = KeyboardShortcutPartial;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        conundrum::ecosystem::db::tables::DatabaseTable::KeyboardShortcut
    }

    fn merge_keys() -> &'static [&'static str] {
        &["action"]
    }

    fn primary_key() -> &'static str {
        "action"
    }

    fn primary_value(&self) -> String {
        self.action.to_string()
    }
}

impl<'a> DBSchema<'a> for KeyboardShortcut {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(EcosystemApplicationAction::field_definition("action", false)),
                Arc::new(String::field_definition("key", false)),
                Arc::new(bool::field_definition("shift", false)),
                Arc::new(bool::field_definition("meta", false)),
                Arc::new(bool::field_definition("alt", false)),
                Arc::new(bool::field_definition("ctrl", false)),])
    }
}
