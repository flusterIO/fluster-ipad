use std::collections::HashMap;

use conundrum::lang::constants::file_types::ParsableFileType;
use strum::IntoEnumIterator;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct SyncContext {
    pub update_count: HashMap<ParsableFileType, u32>,
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
