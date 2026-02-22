uniffi::setup_scaffolding!();

use hayagriva::{
    citationberg::{IndependentStyle, LocaleFile},
    io::from_biblatex_str,
    lang::{SentenceCase, TitleCase},
    BibliographyDriver, BibliographyRequest, BufWriteFormat, CitationItem, CitationRequest, Entry,
};
use std::{ops::Index, sync::Arc};

use crate::{data::constants::RenderMethod, string_utils::split_biblatex_file_by_entries};
mod data;
mod string_utils;
mod test_utils;

// ... classes with methods ...
#[derive(uniffi::Object, Debug)]
pub struct BibEntryData {
    body: String,
    entry: Option<Entry>,
}

fn bib_entry_to_entry(body: &str) -> Option<Entry> {
    if let Ok(res) = from_biblatex_str(body) {
        let key = res.keys().collect::<Vec<&str>>()[0];
        let entry = res.get(key).cloned();
        return entry;
    }
    None
}

fn render_method_to_format(render_method: RenderMethod) -> BufWriteFormat {
    match render_method {
        RenderMethod::Plaintext => hayagriva::BufWriteFormat::Plain,
        RenderMethod::Html => hayagriva::BufWriteFormat::Html,
    }
}

#[uniffi::export]
impl BibEntryData {
    // Constructors need to be annotated as such
    #[uniffi::constructor]
    pub fn new(body: String) -> Self {
        let entry = bib_entry_to_entry(&body);
        Self { body, entry }
    }

    #[uniffi::method]
    pub fn format_bibliography_citation(
        &self,
        csl_content: String,
        csl_locale: String,
        render_method: RenderMethod,
    ) -> Option<String> {
        if let Some(entry) = &self.entry {
            let locale_files = [LocaleFile::from_xml(&csl_locale).unwrap().into()];
            let style = IndependentStyle::from_xml(&csl_content).unwrap();

            let mut driver = BibliographyDriver::new();
            let items = vec![CitationItem::with_entry(entry)];
            driver.citation(CitationRequest::from_items(items, &style, &locale_files));
            let result = driver.finish(BibliographyRequest {
                style: &style,
                locale: None,
                locale_files: &locale_files,
            });

            if let Some(bib) = result.bibliography {
                if bib.items.is_empty() {
                    None
                } else {
                    let mut html_output = String::new();

                    bib.items
                        .index(0)
                        .content
                        .write_buf(&mut html_output, render_method_to_format(render_method))
                        .unwrap();
                    Some(html_output)
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    #[uniffi::method]
    pub fn format_inline_citation(
        &self,
        csl_content: String,
        csl_locale: String,
        render_method: RenderMethod,
    ) -> Option<String> {
        if let Some(entry) = &self.entry {
            let locale_files = [LocaleFile::from_xml(&csl_locale).unwrap().into()];
            let style = IndependentStyle::from_xml(&csl_content).unwrap();

            let mut driver = BibliographyDriver::new();
            let items = vec![CitationItem::with_entry(entry)];
            driver.citation(CitationRequest::from_items(items, &style, &locale_files));
            let result = driver.finish(BibliographyRequest {
                style: &style,
                locale: None,
                locale_files: &locale_files,
            });
            if result.citations.is_empty() {
                None
            } else {
                let mut output = String::new();
                let _ = result
                    .citations
                    .index(0)
                    .citation
                    .write_buf(&mut output, render_method_to_format(render_method));

                Some(output)
            }
        } else {
            None
        }
    }

    #[uniffi::method]
    pub fn get_citation_key(&self) -> Option<String> {
        self.entry.clone().map(|x| x.key().to_string())
    }

    #[uniffi::method]
    pub fn get_title(&self) -> Option<String> {
        if let Some(entry) = &self.entry {
            if let Some(title) = entry.title() {
                let formatted = title.format_title_case(TitleCase::new());
                Some(formatted)
            } else {
                None
            }
        } else {
            None
        }
    }

    #[uniffi::method]
    pub fn get_note(&self) -> Option<String> {
        if let Some(entry) = &self.entry {
            if let Some(title) = entry.note() {
                let formatted = title.format_sentence_case(SentenceCase::new());
                Some(formatted)
            } else {
                None
            }
        } else {
            None
        }
    }

    #[uniffi::method]
    pub fn get_abstract(&self) -> Option<String> {
        if let Some(entry) = &self.entry {
            if let Some(title) = entry.abstract_() {
                let formatted = title.format_sentence_case(SentenceCase::new());
                Some(formatted)
            } else {
                None
            }
        } else {
            None
        }
    }

    #[uniffi::method]
    pub fn get_authors(&self) -> Option<Vec<String>> {
        if let Some(entry) = &self.entry {
            if let Some(people) = entry.authors() {
                let mut res: Vec<String> = Vec::new();
                for person in people {
                    res.push(person.name.clone());
                }
                Some(res)
            } else {
                None
            }
        } else {
            None
        }
    }

    #[uniffi::method]
    pub fn get_url(&self) -> Option<String> {
        if let Some(entry) = &self.entry {
            if let Some(_url) = entry.url() {
                let u = _url.to_string();
                Some(u)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[uniffi::export]
pub fn parse_biblatex_string(biblatex_content: String) -> Vec<Arc<BibEntryData>> {
    split_biblatex_file_by_entries::split_biblatex_to_raw_strings(&biblatex_content)
        .iter()
        .map(|entry_string| Arc::new(BibEntryData::new(entry_string.clone())))
        .collect::<Vec<Arc<BibEntryData>>>()
}

#[cfg(test)]
mod tests {
    use std::ops::Index;

    use crate::test_utils::get_bib_test_content::{get_test_csl_content, get_test_csl_locale};

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
        let csl = get_test_csl_content(None);
        let csl_locale = get_test_csl_locale();

        let formatted_citation =
            entry.format_inline_citation(csl.clone(), csl_locale.clone(), RenderMethod::Html);

        assert!(
            formatted_citation.is_some(),
            "Gets formatted citation when a bibliography entry is present."
        );
        let formatted_citation =
            entry.format_inline_citation(csl, csl_locale, RenderMethod::Plaintext);

        assert!(
            formatted_citation.is_some(),
            "Gets formatted citation when a bibliography entry is present."
        )
    }

    #[test]
    pub fn formats_bibliography_citation() {
        let test_content = test_utils::get_bib_test_content::get_bib_test_content();
        let entries = parse_biblatex_string(test_content);
        let entry = entries.index(0);
        let csl = get_test_csl_content(None);
        let csl_locale = get_test_csl_locale();

        let formatted_citation =
            entry.format_bibliography_citation(csl.clone(), csl_locale.clone(), RenderMethod::Html);
        println!(
            "Formatted Citation (html): {}",
            formatted_citation.clone().unwrap_or("None".to_string())
        );
        assert!(
            formatted_citation.is_some(),
            "Gets formatted citation when a bibliography entry is present."
        );
        let formatted_citation =
            entry.format_bibliography_citation(csl, csl_locale, RenderMethod::Plaintext);
        println!(
            "Formatted Citation (plain): {}",
            formatted_citation.clone().unwrap_or("None".to_string())
        );
        assert!(
            formatted_citation.is_some(),
            "Gets formatted citation when a bibliography entry is present."
        )
    }
}
