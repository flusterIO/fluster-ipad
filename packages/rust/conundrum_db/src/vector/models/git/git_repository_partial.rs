use std::sync::Arc;

use arrow_schema::{DataType, Field};
use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use fake::Dummy;

use crate::vector::{
    database::db_traits::db_field::DatabaseField,
    models::{primitives::db_id::DatabaseId, vector::vector::DBVector},
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct GitRepositoryPartial {
    /// Will match the root of the workspace if this is a workspace repository.
    pub fs_path: Option<String>,
    pub url: Option<String>,
    pub id: DatabaseId,
    /// A descriptive label used for both the UI and as further information for
    /// AI.
    pub label: Option<String>,
    is_workspace: Option<bool>,
    pub allow_ai_access: Option<bool>,
}

impl<'a> DBSchema<'a> for GitRepositoryPartial {}
