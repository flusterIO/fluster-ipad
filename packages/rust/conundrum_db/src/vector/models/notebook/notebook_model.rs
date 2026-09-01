use std::sync::Arc;

use arrow_schema::Field;
use conundrum::{
    ecosystem::db::{db_traits::db_entity::DBSchema, tables::DatabaseTable},
    lang::runtime::run_conundrum::ParseConundrumOptions,
    lifted_models::text::json_content::JsonContent,
};
use conundrum_macros::{DatabaseEntity, DatabaseModel};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    notebook::ipynb_content::IpynbContent,
    text::{
        cdrm::cdrm_content::CdrmContent,
        text_based_content::{text_based_chunk::TextBasedChunk, text_based_content::TextBasedContent},
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy, DatabaseEntity)]
#[serde(transparent)]
#[db(table = DatabaseTable::Notebook, unit = TextBasedContent)]
pub struct NotebookModel<'a>(TextBasedContent<'a, IpynbContent, TextBasedChunk, ParseConundrumOptions>);
