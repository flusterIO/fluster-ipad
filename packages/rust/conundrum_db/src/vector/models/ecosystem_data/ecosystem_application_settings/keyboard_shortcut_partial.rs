use std::sync::Arc;

use conundrum::ecosystem::db::{
    db_traits::{db_entity::DBSchema, db_field::DatabaseField},
    tables::DatabaseTable,
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::ecosystem_data::ecosystem_application_settings::ecosystem_application_action::EcosystemApplicationAction;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
pub struct KeyboardShortcutPartial {
    #[db(primary)]
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
