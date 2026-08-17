use std::sync::Arc;

use conundrum::{
    ecosystem::db::db_traits::{
        db_entity::DBSchema,
        db_field::{DatabaseField, DatabaseFieldLarge},
    },
    lifted_models::primitives::{bytes::Bytes, db_id::DatabaseId},
};

use crate::vector::models::ai::ai_interactions::AIInteractions;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, specta::Type, fake::Dummy)]
pub struct PdfEntity {
    pub id: DatabaseId,
    pub title: Option<String>,
    pub data: Bytes,
    /// The text extracted from the pdf. It will be null if extraction fails.
    pub text: Option<String>,
    pub ai: AIInteractions,
}

impl<'a> DBSchema<'a> for PdfEntity {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(String::field_definition("title", true)),
                Arc::new(Bytes::field_definition_large("data", false)),
                Arc::new(AIInteractions::field_definition("ai", false))])
    }
}

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
