use std::sync::Arc;

use arrow_schema::{DataType, Field};
use conundrum::ecosystem::db::db_traits::{db_entity::DBSchema, db_field::DatabaseField};
use conundrum_macros::DBSchema;
use fake::Dummy;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::vector::models::{ai::ai_interactions::AIInteractions, workspace::user_workspace::UserWorkspace};

#[derive(Serialize, Deserialize, Clone, Debug, Type, Dummy, DBSchema)]
pub struct UserWorkspacePartial {
    /// The path to the root of the workspace and the primary key for the
    /// workspace. This is still required to update the proper item.
    pub root: String,
    pub label: Option<String>,
    pub respect_gitignore: Option<bool>,
    pub ignore_hidden: Option<bool>,
    pub resource_dir: Option<String>,
    pub ai: Option<AIInteractions>,
}
