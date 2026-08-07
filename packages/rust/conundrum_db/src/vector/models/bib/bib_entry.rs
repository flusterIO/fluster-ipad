use conundrum::{
    bibliography::{bib_entry::BibEntry, split_bibtex_by_entries::split_biblatex_to_raw_strings},
    ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult},
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::vector::models::{date_time::date_time::DateTime, primitives::db_id::DatabaseId};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BibEntryModel {
    pub id: DatabaseId,
    /// The key of the biblatex entry, used as an id for this entry in the
    /// user's notes.
    pub key: String,
    /// The raw biblatex string for a single entry.
    pub biblatex: String,
    /// A boolean indicating if this literature was already reviewed by the
    /// user.
    pub read: bool,
    /// The time this bibliography entry was created by the user.
    pub ctime: DateTime,
    /// The time the biblatex content was last updated by the user.
    pub utime: DateTime,
}

impl BibEntryModel {
    /// Takes a biblatex string and returns a vec of all the items contained
    /// within the string. This will always return an array, even it there's
    /// one, because it's Rust and we can't python s--t.
    pub fn from_biblatex(biblatex: &str) -> DatabaseResult<Vec<BibEntryModel>> {
        let strings = split_biblatex_to_raw_strings(biblatex);
        let r = strings.par_iter()
                       .map(|s| {
                           let bib_entry = BibEntry::from(s.clone());
                           let parsed_entry = bib_entry.to_entry().map_err(DatabaseError::ConundrumError)?;
                           let k = parsed_entry.key();
                           Ok(BibEntryModel { id: DatabaseId::new(),
                                              key: k.to_string(),
                                              biblatex: s.to_string(),
                                              read: false,
                                              ctime: DateTime::new_now(),
                                              utime: DateTime::new_now() })
                       })
                       .collect::<DatabaseResult<Vec<BibEntryModel>>>()?;
        Ok(r)
    }
}
