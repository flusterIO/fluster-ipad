use std::{fmt::Display, path::PathBuf};

use conundrum::ecosystem::error_handling::conundrum_fs_error::{ConundrumFSError, ConundrumFSResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CDRMPathBuf(PathBuf);

impl CDRMPathBuf {
    pub fn from_pathbuf(pb: PathBuf) -> CDRMPathBuf {
        CDRMPathBuf(pb)
    }

    pub fn try_to_string(&self) -> ConundrumFSResult<String> {
        Ok(self.0.to_str().ok_or(ConundrumFSError::PathSerializationError)?.to_string())
    }
}
