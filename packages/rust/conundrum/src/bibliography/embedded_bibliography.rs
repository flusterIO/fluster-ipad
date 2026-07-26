pub use rust_embed::Embed;

use crate::bibliography::embedded_csl_file::EmbeddedCSLFile;

#[derive(Embed)]
#[folder = "src/embedded/bibliography/"]
pub struct EmbeddedBibliography;

impl EmbeddedBibliography {
    pub fn read_string(fp: &str) -> Option<String> {
        if let Some(res) = Self::get(fp)
           && let Ok(body) = std::str::from_utf8(res.data.as_ref())
        {
            Some(body.to_string())
        } else {
            None
        }
    }

    /// TODO:
    /// Embed some other common locales and actually make this framework
    /// accessible.
    pub fn read_csl_locale_file() -> String {
        Self::read_string("csl_locale/en_us.xml").unwrap()
    }

    pub fn get_embedded_csl_file(embedded: EmbeddedCSLFile) -> String {
        Self::read_string(embedded.to_string().as_str()).unwrap()
    }
}
