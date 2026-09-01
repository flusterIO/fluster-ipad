use std::sync::Arc;

use arrow_schema::Field;
use conundrum::{
    ecosystem::db::{db_traits::db_entity::DBSchema, tables::DatabaseTable},
    lang::runtime::run_conundrum::ParseConundrumOptions,
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::text::{
    cdrm::cdrm_content::CdrmContent,
    text_based_content::{text_based_chunk::TextBasedChunk, text_based_content::TextBasedContent},
};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy, DatabaseEntity, specta::Type)]
#[serde(transparent)]
#[db(table = DatabaseTable::HTML, unit = TextBasedContent)]
pub struct HTMLModel<'a>(TextBasedContent<'a, CdrmContent, TextBasedChunk, ParseConundrumOptions>);
