use std::sync::Arc;

use crate::ecosystem::db::db_traits::db_entity::DBSchema;
use crate::ecosystem::db::db_traits::db_field::DatabaseField;
use crate::lifted_models::primitives::db_id::DatabaseId;
use conundrum_macros::DBSchema;
use serde::{Deserialize, Serialize};

/// Deprecated in favor of doing s--t that make sense.
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, fake::Dummy, DBSchema)]
#[db(source_crate = true)]
pub struct ChatConversationPartial {
    pub id: DatabaseId,
    pub label: Option<String>,
    pub requires_label_update: Option<bool>,
    pub desc: Option<String>,
}
