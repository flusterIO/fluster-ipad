use std::sync::Arc;

use conundrum::{
    ecosystem::db::{
        db_traits::{
            db_entity::DBSchema,
            db_field::{DatabaseField, DatabaseFieldLarge},
        },
        tables::DatabaseTable,
    },
    lang::runtime::run_conundrum::ParseConundrumOptions,
    lifted_models::primitives::{bytes::Bytes, db_id::DatabaseId},
};
use conundrum_macros::DatabaseEntity;

use crate::vector::models::{
    ai::ai_interactions::AIInteractions,
    binary::{
        binary_based_content::BinaryBasedContent,
        binary_based_content_trait::BinaryBasedContent as BinaryBasedContentTrait,
    },
    pdf::pdf_binary::PdfBinary,
    text::text_based_content::text_based_chunk::TextBasedChunk,
};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, specta::Type, fake::Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::Pdf, unit = BinaryBasedContent<PdfBinary, TextBasedChunk, ParseConundrumOptions>)]
pub struct PdfModel(BinaryBasedContent<PdfBinary, TextBasedChunk, ParseConundrumOptions>);

// impl FromFilePath<DatabaseError> for PdfModel {
//     async fn from_file_path(fp: impl AsRef<std::path::Path>) -> Result<Self,
// DatabaseError>         where Self: Sized {
//             let data = tokio::fs::read(fp)
//                 .await
//                 .map_err(|e| {
//                     log::error!("Notebook File System Error: {:?}", e);
//
// DatabaseError::FileSystemError(conundrum::ecosystem::error_handling::conundrum_fs_error::ConundrumFSError::FsError(e.
// to_string()))                 })?;
//     }
// }
