#[derive(Default, Debug)]
pub struct CitationList {
    /// A vector of the citationKeys
    pub data: Vec<String>,
}

impl CitationList {
    pub fn index_of_citation_key(&self, citation_key: &str) -> Option<usize> {
        for (i, s) in self.data.iter().enumerate() {
            if *s == citation_key {
                return Some(i);
            }
        }
        None
    }

    pub fn safely_append_item(&mut self, citation_key: &str) {
        if self.index_of_citation_key(citation_key).is_none() {
            self.data.push(citation_key.to_string());
        }
    }

    /// Appends the item if it does not exist, and returns the index of the
    /// citation.
    pub fn get_item_index_and_append(&mut self, citation_key: &str) -> usize {
        if let Some(idx) = self.index_of_citation_key(citation_key) {
            idx
        } else {
            self.data.push(citation_key.to_string());
            self.data.len() - 1
        }
    }

    pub fn merge_child_state(&mut self, child_state: &Self) {
        for item in child_state.data.iter() {
            self.safely_append_item(item.as_str());
        }
    }
}
