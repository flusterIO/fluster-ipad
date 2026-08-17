use conundrum::ecosystem::db::db_traits::async_traits::actionable_request::ActionableRequest;
use conundrum::ecosystem::error_handling::conundrum_fs_error::ConundrumFSError;
use conundrum::ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult};
use conundrum::lang::constants::file_types::ParsableFileType;
use std::str::FromStr;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, strum_macros::Display)]
pub enum PathSourceType {
    #[serde(rename = "file")]
    #[strum(to_string = "file")]
    File,
    #[serde(rename = "directory")]
    #[strum(to_string = "directory")]
    Dir,
    #[serde(rename = "any")]
    #[strum(to_string = "any")]
    Any,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct PathValidationRequest {
    pub path: String,
    /// This only makes sense with an empty permitted_types array. Otherwise the
    /// source_type is obviously a file. This will however validate 'any' paths
    /// as being either a file or directory as expected.
    pub source_type: PathSourceType,
    /// An empty array will default to any file type. Not just any parsable file
    /// type.
    pub permitted_types: Vec<ParsableFileType>,
}

impl PathValidationRequest {
    pub fn matches_permitted_types(&self, p: &std::path::Path) -> DatabaseResult<bool> {
        if self.permitted_types.is_empty() {
            Ok(true)
        } else {
            if let Some(ext) = p.extension() {
                if let Some(file_ext_str) = ext.to_str() {
                    if let Ok(pf) = ParsableFileType::from_str(file_ext_str) {
                        Ok(true)
                    } else {
                        Err(DatabaseError::FileSystemError(ConundrumFSError::InvalidExtension { target_file:
                                                                                                    self.path.clone() }))
                    }
                } else {
                    Err(DatabaseError::FileSystemError(ConundrumFSError::NoFileExtensionFound { target_file:
                                                                                                    self.path.clone() }))
                }
            } else {
                Err(DatabaseError::FileSystemError(ConundrumFSError::NoFileExtensionFound { target_file:
                                                                                                self.path.clone() }))
            }
        }
    }

    fn match_source_type(&self, pb: &std::path::Path) -> DatabaseResult<bool> {
        if let Ok(meta) = pb.metadata() {
            match self.source_type {
                PathSourceType::Dir => {
                    let is_dir = meta.is_dir();
                    Ok(is_dir)
                }
                PathSourceType::File => {
                    let is_file = meta.is_file();
                    Ok(is_file)
                }
                PathSourceType::Any => {
                    log::warn!("This should never be reached. Something went wrong while validating this path.");
                    Err(DatabaseError::FileSystemError(ConundrumFSError::InvalidFileMeta { target_file:
                                                                                               self.path.clone() }))
                }
            }
        } else {
            Err(DatabaseError::FileSystemError(ConundrumFSError::InvalidFileMeta { target_file: self.path.clone() }))
        }
    }
}

impl ActionableRequest<bool> for PathValidationRequest {
    async fn execute_request(&self) -> DatabaseResult<bool> {
        let exists = tokio::fs::try_exists(&self.path).await.map_err(|e| {
                                                                 log::error!("File system error: {:?}", e);
                                                                 DatabaseError::FileSystemError(
                                                                     ConundrumFSError::FsError("Random tokio error. Check the logs.".to_string())
                                                                 )
                                                             })?;

        let p = std::path::Path::new(&self.path);
        match exists {
            true => match self.source_type {
                PathSourceType::Any => self.matches_permitted_types(p),
                _ => self.match_source_type(p),
            },
            false => Err(DatabaseError::FileSystemError(ConundrumFSError::PathDoesntExist(self.path.clone()))),
        }
    }
}
