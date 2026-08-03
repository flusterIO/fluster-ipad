use conundrum::{
    ecosystem::error_handling::conundrum_fs_error::{ConundrumFSError, ConundrumFSResult},
    lang::constants::file_types::ParsableFileType,
};
use ignore::types::{Types, TypesBuilder};
use strum::IntoEnumIterator;

pub fn get_all_parsable_ignore_types() -> ConundrumFSResult<Types> {
    let mut t = TypesBuilder::new();
    for pt in ParsableFileType::iter() {
        let (k, v) = pt.to_ignore_types();
        t.add(k, v);
    }
    t.build().map_err(|e| {
                 log::error!("Error: {:?}", e);
                 ConundrumFSError::GeneralFSError
             })
}
