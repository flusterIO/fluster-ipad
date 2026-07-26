use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BibEntryRenderMethod {
    Html,
    Plaintext,
}

impl BibEntryRenderMethod {
    pub fn to_format(&self) -> hayagriva::BufWriteFormat {
        match self {
            Self::Plaintext => hayagriva::BufWriteFormat::Plain,
            Self::Html => hayagriva::BufWriteFormat::Html,
        }
    }
}
