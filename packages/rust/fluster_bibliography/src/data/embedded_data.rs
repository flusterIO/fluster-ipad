use core::str;

use rust_embed::RustEmbed;

use crate::data::embedded_csl_file::EmbeddedCslFile;

#[derive(RustEmbed)]
#[folder = "src/data/embedded"]
pub struct EmbeddedData;

impl EmbeddedData {
    pub fn get_csl_locale() -> String {
        let x = EmbeddedData::get("csl_locale/en_us.xml").unwrap().data;
        let y = str::from_utf8(&x).unwrap();
        y.to_string()
    }

    pub fn get_embedded_csl_file(f: EmbeddedCslFile) -> String {
        let p = format!("csl/{}", f.to_string());
        let x = EmbeddedData::get(&p).unwrap().data;
        let y = str::from_utf8(&x).unwrap();
        y.to_string()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn gets_embedded_data() {
        let locale = EmbeddedData::get_csl_locale();

        // assert!(locale.is_some(), "Gets embedded data.");
    }

    #[test]
    fn gets_embedded_csl_files() {
        for f in EmbeddedCslFile::iter() {
            let f = EmbeddedData::get_embedded_csl_file(f);
        }

        // assert!(locale.is_some(), "Gets embedded data.");
    }
}
