use std::sync::Arc;

use arrow_schema::{DataType, Field};
use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use fake::Dummy;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::vector::models::workspace::user_workspace::UserWorkspace;

#[derive(Serialize, Deserialize, Clone, Debug, Type, Dummy)]
pub struct UserWorkspacePartial {
    /// The path to the root of the workspace and the primary key for the
    /// workspace. This is still required to update the proper item.
    pub root: String,
    pub label: Option<String>,
    pub bib_paths: Option<Vec<String>>,
    pub respect_gitignore: Option<bool>,
    pub ignore_hidden: Option<bool>,
    pub resource_dir: Option<String>,
}

impl<'a> DBSchema<'a> for UserWorkspacePartial {}
