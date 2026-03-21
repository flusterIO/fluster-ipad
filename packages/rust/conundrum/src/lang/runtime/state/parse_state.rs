use crate::output::parsing_result::mdx_parsing_result::MdxParsingResult;

#[derive(Debug, Default)]
pub struct ParseState {
    pub data: MdxParsingResult,
}

impl ParseState {
    /// Applies the nested state found within a child element to the parent
    /// state.
    pub fn apply_nested_state(&mut self, s: Self) {
        s.data.citations.iter().for_each(|cit| {
                                   self.data.citations.push(cit.clone());
                               });
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
}
