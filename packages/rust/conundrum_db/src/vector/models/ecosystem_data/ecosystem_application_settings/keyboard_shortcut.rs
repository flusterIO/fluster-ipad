use std::sync::Arc;

use conundrum::{
    ecosystem::db::{
        db_traits::{
            db_entity::{DBEntity, DBSchema},
            db_field::DatabaseField,
        },
        tables::DatabaseTable,
    },
    impl_default_crud,
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::ecosystem_data::ecosystem_application_settings::ecosystem_application_action::EcosystemApplicationAction;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::KeyboardShortcut)]
pub struct KeyboardShortcut {
    #[db(primary)]
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
