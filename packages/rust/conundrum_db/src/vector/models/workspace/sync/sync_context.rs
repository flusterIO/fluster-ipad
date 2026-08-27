use std::collections::HashMap;

use conundrum::lang::constants::file_types::ParsableFileType;
use strum::IntoEnumIterator;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct SyncContext {
    pub update_count: HashMap<ParsableFileType, u32>,
}

impl SyncContext {
    pub fn increment_parsable_file_count(&mut self, pf: ParsableFileType) {
        if let Some(existing) = self.update_count.get(&pf) {
            self.update_count.insert(pf.clone(), existing + 1);
        } else {
            self.update_count.insert(pf.clone(), 1);
        }
    }
}

impl Default for SyncContext {
    fn default() -> Self {
        let mut update_count = HashMap::new();
        for pf in ParsableFileType::iter() {
            update_count.insert(pf, 0);
        }
        Self { update_count }
    }
}
