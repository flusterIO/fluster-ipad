use serde::{Deserialize, Serialize};

use crate::vector::models::{
    bib::bib_entry::BibEntryModel,
    taggables::{subject::Subject, tag::Tag, topic::Topic},
};

/// ## Bibliography
///
/// Most of the time this will be used in one of two use cases:
///
/// - Preparing a paper for submission, separating your bibliography entries as
///   you need them.
/// - Using AI to generate a list of bibliography entries that were cited by a
///   specific article. This feature is a work in progress, as I'm without
///   internet, and unable to reliably work with AI, despite ironically building
///   an AI focused framework.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bibliography {
    pub label: String,
    pub desc: Option<String>,
    pub entries: Vec<BibEntryModel>,
    pub topic: Option<Topic>,
    pub subject: Option<Subject>,
    pub tags: Option<Tag>,
    /// An optional article that this bibliography was generated from.
    ///
    /// This is ***not*** optional for AI. AI models must always include the
    /// source article when they are used to generate a bibliography if a single
    /// article was used to produce the list.
    pub source_article: Option<BibEntryModel>,
}
