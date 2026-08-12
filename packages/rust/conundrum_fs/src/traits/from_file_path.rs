use std::path::Path;

use conundrum::ecosystem::error_handling::conundrum_fs_error::ConundrumFSError;

pub trait FromFilePath<ErrorType = ConundrumFSError> {
    fn from_file_path(fp: impl AsRef<Path>) -> Result<Self, ErrorType>
        where Self: Sized;
}
