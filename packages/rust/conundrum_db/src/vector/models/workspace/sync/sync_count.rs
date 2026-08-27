use std::collections::HashMap;

use conundrum::lang::constants::file_types::ParsableFileType;
use strum::IntoEnumIterator;

use crate::vector::models::workspace::sync::ir::generic_tag_input::GenericTagInput;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct SyncCount {
    pub update_count: HashMap<ParsableFileType, u32>,
    pub tags: u32,
    pub topics: u32,
    pub subjects: u32,
}

impl SyncCount {
    pub fn increment_parsable_file_count(&mut self, pf: ParsableFileType) {
        if let Some(existing) = self.update_count.get(&pf) {
            self.update_count.insert(pf.clone(), existing + 1);
        } else {
            self.update_count.insert(pf.clone(), 1);
        }
    }
}

impl Default for SyncCount {
    fn default() -> Self {
        let mut update_count = HashMap::new();
        for pf in ParsableFileType::iter() {
            update_count.insert(pf, 0);
        }
        Self { update_count,
               tags: 0,
               topics: 0,
               subjects: 0 }
    }
}
