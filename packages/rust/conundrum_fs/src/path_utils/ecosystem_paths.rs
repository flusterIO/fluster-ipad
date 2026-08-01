use std::path::PathBuf;

use crate::errors::conundrum_fs_error::{ConundrumFSError, ConundrumFSResult};

pub fn get_app_data_dir() -> ConundrumFSResult<PathBuf> {
    if let Some(d) = dirs::data_local_dir() {
        Ok(d.join("conundrum"))
    } else {
        dirs::data_dir().map(|x| x.join("conundrum")).ok_or(ConundrumFSError::InvalidDataDirectory)
    }
}

pub fn get_app_database_dir() -> ConundrumFSResult<PathBuf> {
    Ok(get_app_data_dir()?.join("database"))
}


