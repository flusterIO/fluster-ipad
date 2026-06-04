use crate::{errors::DocGenResult, traits::DocGenTemplate};

pub struct GeneralFileType {
    /// The path relative to the template directory.
    pub from: String,
    /// The monorepo root relative path as a unix compatible string, because I
    /// give up on all that `.join` shit for an app that only runs on Unix
    /// so far...
    pub to: String,
}

impl GeneralFileType {
    fn gather_data(from: String, to: String) -> Self {
        Self { from,
               to }
    }

    fn write_data(&self) -> DocGenResult<()> {
        let tem
    }
}
