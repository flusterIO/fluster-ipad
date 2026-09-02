use conundrum::{
    ecosystem::db::tables::DatabaseTable,
    lang::runtime::run_conundrum::ParseConundrumOptions,
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};
use conundrum_fs::models::user_workspace::workspace_relative_path::WorkspaceRelativePath;
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    ai::ai_generated_status::AIGeneratedStatus,
    taggables::{subject::Subject, tag_list::TagList, topic::Topic},
    text::{
        text_based_content::{text_based_chunk::TextBasedChunk, text_based_content::TextBasedContent},
        typst::{parse_typst_opts::ParseTypstOpts, typst_content::TypstContent},
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy, DatabaseEntity, specta::Type)]
#[serde(transparent)]
#[db(table = DatabaseTable::Typst, unit = TextBasedContent<TypstContent, TextBasedChunk, ParseTypstOpts>)]
pub struct TypstModel(TextBasedContent<TypstContent, TextBasedChunk, ParseTypstOpts>);
