uniffi::setup_scaffolding!();

use std::ops::Index;

use citeproc::csl::style;
use hayagriva::{
    citationberg::{Bibliography, IndependentStyle, LocaleFile, Style, StyleClass},
    io::from_biblatex_str,
    BibliographyDriver, BibliographyRequest, CitationItem, CitationRequest, CitePurpose, Entry,
    Formatting,
};
use serde::{Deserialize, Serialize};

use crate::{
    data::{embedded_csl_file::EmbeddedCslFile, embedded_data::EmbeddedData},
    string_utils::split_biblatex_file_by_entries,
};
mod data;
mod string_utils;
mod test_utils;

// ... classes with methods ...
#[derive(uniffi::Object, Debug, Deserialize, Serialize)]
pub struct BibEntryData {
    body: String,
}

fn bib_entry_to_entry(entry: &BibEntryData) -> Option<Entry> {
    if let Ok(res) = from_biblatex_str(&entry.body) {
        let key = res.keys().collect::<Vec<&str>>()[0];
        let entry = res.get(key).cloned();
        return entry;
    }
    None
}

// // Return a json serializable string. Use Rust to read values directly from the bib entry if needed
// // instead of parsing it into a map because of the awkward, largely unknown shape.
// #[uniffi::export]
// pub fn parse_bib_entry(bib_entry_body: String) {}

#[uniffi::export]
impl BibEntryData {
    // Constructors need to be annotated as such
    #[uniffi::constructor]
    pub fn new(body: String) -> Self {
        Self { body }
    }

    pub fn format_bibliography_citation(&self, csl_format: EmbeddedCslFile) -> Option<String> {
        if let Some(entry) = bib_entry_to_entry(self) {
            let locale = EmbeddedData::get_csl_locale();
            let locale_files = [LocaleFile::from_xml(&locale).unwrap().into()];
            let style_file_content = EmbeddedData::get_embedded_csl_file(csl_format);
            let mut style = IndependentStyle::from_xml(&style_file_content).unwrap();
            let s = Style::from_xml(&style_file_content).unwrap();

            print!("Settings: {:#?}", style.settings);

            style.settings.class = StyleClass::Note;

            print!("Settings: {:#?}", style.settings);

            let mut driver = BibliographyDriver::new();
            let items = vec![CitationItem::with_entry(&entry)];
            driver.citation(CitationRequest::from_items(items, &style, &locale_files));
            let result = driver.finish(BibliographyRequest {
                style: &style,
                locale: None,
                locale_files: &locale_files,
            });

            // println!("Result: {:#?}", result);
            if let Some(bib) = result.bibliography {
                if bib.items.is_empty() {
                    None
                } else {
                    let mut html_output = String::new();

                    bib.items
                        .index(0)
                        .content
                        .write_buf(&mut html_output, hayagriva::BufWriteFormat::Html)
                        .unwrap();
                    println!("HTML: {}", html_output);
                    Some(html_output)
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn format_inline_citation(&self, csl_format: EmbeddedCslFile) -> Option<String> {
        if let Some(entry) = bib_entry_to_entry(self) {
            let locale = EmbeddedData::get_csl_locale();
            let locale_files = [LocaleFile::from_xml(&locale).unwrap().into()];
            let style = EmbeddedData::get_embedded_csl_file(csl_format);
            let style = IndependentStyle::from_xml(&style).unwrap();

            let mut driver = BibliographyDriver::new();
            let items = vec![CitationItem::with_entry(&entry)];
            driver.citation(CitationRequest::from_items(items, &style, &locale_files));
            let result = driver.finish(BibliographyRequest {
                style: &style,
                locale: None,
                locale_files: &locale_files,
            });
            // println!("Result: {:#?}", result);
            if result.citations.is_empty() {
                None
            } else {
                // result.citations.index(0).dis
                // result.dis
                Some(result.citations.index(0).citation.to_string())
            }
        } else {
            None
        }
    }
}

pub fn parse_biblatex_string(biblatex_content: String) -> Vec<BibEntryData> {
    split_biblatex_file_by_entries::split_biblatex_to_raw_strings(&biblatex_content)
        .iter()
        .map(|entry_string| BibEntryData::new(entry_string.clone()))
        .collect::<Vec<BibEntryData>>()
}

#[cfg(test)]
mod tests {
    use std::ops::Index;

    use super::*;

    #[test]
    pub fn parses_biblatex_content() {
        let test_content = test_utils::get_bib_test_content::get_bib_test_content();
        let entries = parse_biblatex_string(test_content);
        assert!(entries.len() == 3, "Returns proper number of items");
    }

    #[test]
    pub fn formats_inline_citation() {
        let test_content = test_utils::get_bib_test_content::get_bib_test_content();
        let entries = parse_biblatex_string(test_content);
        let entry = entries.index(0);

        let formatted_citation = entry.format_inline_citation(EmbeddedCslFile::Apa);

        assert!(
            formatted_citation.is_some(),
            "Gets formatted citation when a bibliography entry is present."
        )
        // assert!(entries.len() == 3, "Returns proper number of items");
    }

    #[test]
    pub fn formats_bibliography_citation() {
        let test_content = test_utils::get_bib_test_content::get_bib_test_content();
        let entries = parse_biblatex_string(test_content);
        let entry = entries.index(0);

        let formatted_citation =
            entry.format_bibliography_citation(EmbeddedCslFile::AmericanMedicalAssociation);

        println!(
            "Formatted Citation: {}",
            formatted_citation.clone().unwrap_or("None".to_string())
        );

        assert!(
            formatted_citation.is_some(),
            "Gets formatted citation when a bibliography entry is present."
        )
        // assert!(entries.len() == 3, "Returns proper number of items");
    }
}
