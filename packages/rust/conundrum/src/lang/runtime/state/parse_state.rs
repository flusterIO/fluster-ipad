use fluster_core_utilities::core_types::syntax::parser_ids::ParserId;
use serde::{Deserialize, Serialize};
use uniffi::Enum;

use crate::{
    lang::runtime::state::citation_list::CitationList, output::parsing_result::mdx_parsing_result::MdxParsingResult,
};

#[typeshare::typeshare]
#[derive(Enum, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum ConundrumModifier {
    HideAiInput,
    PreferMarkdownSyntax,
    /// Useful for search related features, being able to match text without
    /// markdown syntax interfering. Not super useful for much else.
    ForcePlainText,
    /// Set this flag when the output is intended to be consumed by AI, probably
    /// with the 'PreferMarkdownSyntax' flag.
    ForAIInput,

    /// When this component is to be used for search text input, all of the
    /// component jsx and mdx syntax will be removed, leaving only
    /// searchable text.
    ForSearchInput,
}

#[derive(Debug, Default, Clone)]
pub struct ParseState {
    pub data: MdxParsingResult,
    pub bib: CitationList,
    pub modifiers: Vec<ConundrumModifier>,
}

impl ParseState {
    /// Applies the nested state found within a child element to the parent
    /// state.
    pub fn apply_nested_state(&mut self, s: Self) {
        s.data.tags.iter().for_each(|tag| self.data.tags.push(tag.clone()));

        s.data.outgoing_links.iter().for_each(|ol| self.data.outgoing_links.push(ol.clone()));

        if s.data.ignore_all_parsers {
            self.data.ignore_all_parsers = true;
        }

        s.data.ai_secondary_parse_requests.iter().for_each(|pr| {
                                                     self.data.ai_secondary_parse_requests.push(pr.clone());
                                                 });

        s.data.dictionary_entries.iter().for_each(|dict| {
                                            self.data.dictionary_entries.push(dict.clone());
                                        })
    }

    pub fn contains_modifier(&self, modifier: &ConundrumModifier) -> bool {
        self.modifiers.iter().any(|x| x == modifier)
    }

    pub fn should_ignore_parser(&self, id: &ParserId) -> bool {
        self.data.front_matter.as_ref().is_some_and(|fm| fm.ignored_parsers.iter().any(|x| x == &id.to_string()))
    }
}
