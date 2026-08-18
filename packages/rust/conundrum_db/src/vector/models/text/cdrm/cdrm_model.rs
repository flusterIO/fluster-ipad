use std::sync::Arc;

use arrow_schema::Field;
use conundrum::{ecosystem::db::db_traits::db_entity::DBSchema, lang::runtime::run_conundrum::ParseConundrumOptions};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::text::{
    cdrm::{cdrm_chunk::CdrmChunk, cdrm_content::CdrmContent},
    text_based_content::{text_based_chunk::TextBasedChunk, text_based_content::TextBasedContent},
};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy)]
#[serde(transparent)]
pub struct CdrmModel(TextBasedContent<CdrmContent, TextBasedChunk, ParseConundrumOptions>);

impl<'a> DBSchema<'a> for CdrmModel {
    fn arrow_fields() -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<Arc<Field>>> {
        TextBasedContent::<CdrmContent, TextBasedChunk, ParseConundrumOptions>::arrow_fields()
    }
}
