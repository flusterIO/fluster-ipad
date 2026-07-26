use std::ops::Index;

use chrono::{DateTime, Utc};
use hayagriva::{
    BibliographyDriver, BibliographyRequest, CitationItem, CitationRequest, Entry, Rendered,
    citationberg::{IndependentStyle, LocaleFile},
    io::from_biblatex_str,
};

use crate::{
    bibliography::{
        bib_entry_render_method::BibEntryRenderMethod, embedded_bibliography::EmbeddedBibliography,
        embedded_csl_file::EmbeddedCSLFile,
    },
    ecosystem::environment_variables::cdrm_env_variable::CdrmEnvVariable,
    lang::runtime::state::conundrum_error_variant::{ConundrumErrorVariant, ConundrumResult},
};

pub struct BibEntry {
    pub body: String,
    pub ctime: DateTime<Utc>,
    pub utime: DateTime<Utc>,
}

pub struct RenderedBibEntry {
    pub content: String,
    pub format: BibEntryRenderMethod,
    pub ctime: DateTime<Utc>,
    pub utime: DateTime<Utc>,
}

impl BibEntry {
    pub async fn get_csl_file() -> String {
        if let Ok(csl_path) = CdrmEnvVariable::CSLFilePath.read() {
            if let Ok(content) = tokio::fs::read_to_string(csl_path).await {
                return content;
            }
        }
        EmbeddedBibliography::get_embedded_csl_file(EmbeddedCSLFile::default())
    }

    pub fn to_entry(&self) -> ConundrumResult<Entry> {
        if let Ok(res) = from_biblatex_str(&self.body) {
            let key = res.keys().collect::<Vec<&str>>()[0];
            if let Some(k) = res.get(key).cloned() {
                return Ok(k);
            }
        }
        Err(ConundrumErrorVariant::InvalidBiblatex(self.body.clone()))
    }

    pub async fn entries_to_hayagriva(entries: &[Self]) -> ConundrumResult<Rendered> {
        let csl_locale = EmbeddedBibliography::read_csl_locale_file();
        let csl_file = Self::get_csl_file().await;
        let locale_files = [LocaleFile::from_xml(&csl_locale).map_err(|e| ConundrumErrorVariant::InvalidCSL)?.into()];
        let csl_style = IndependentStyle::from_xml(&csl_file).map_err(|e| ConundrumErrorVariant::InvalidCSL)?;
        let mut driver = BibliographyDriver::new();
        let parsed_entries = entries.iter().map(|item| item.to_entry()).collect::<ConundrumResult<Vec<Entry>>>()?;
        let items: Vec<CitationItem<Entry>> = parsed_entries.iter().map(CitationItem::with_entry).collect();

        driver.citation(CitationRequest::from_items(items, &csl_style, &locale_files));

        Ok(driver.finish(BibliographyRequest { style: &csl_style,
                                               locale: None,
                                               locale_files: &locale_files }))
    }

    /// Renders entries for the bibliography at the end of the note or on an
    /// independent page. Use `render_entries_inline` to render the
    /// bibliography citation that belongs inline with the markdown content.
    pub async fn render_entries_for_bib(method: BibEntryRenderMethod,
                                        entries: Vec<Self>)
                                        -> ConundrumResult<Vec<RenderedBibEntry>> {
        let result = Self::entries_to_hayagriva(&entries).await?;

        if let Some(bib) = result.bibliography {
            let mut items = Vec::new();
            for (i, item) in bib.items.iter().enumerate() {
                let input_item = entries.index(i);
                let mut content = String::new();
                if let Err(_) =
                    item.content.write_buf(&mut content, method.to_format()).map_err(|e| {
                                                                                log::error!("Bibliography Error: {:?}",
                                                                                            e);
                                                                                ConundrumErrorVariant::InvalidBiblatex
                                                                            })
                {
                    log::error!("Invalid Biblatex");
                }
                items.push(RenderedBibEntry { content,
                                              ctime: input_item.ctime,
                                              utime: input_item.utime,
                                              format: method.clone() });
            }
            Ok(items)
        } else {
            Err(ConundrumErrorVariant::EmptyBibliography)
        }
    }
}
