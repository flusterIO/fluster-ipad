use serde::{Deserialize, Serialize};

use crate::vector::models::date_time::date_time::DateTime;

#[derive(Serialize, Deserialize, Clone)]
pub struct BibEntryModel {
    /// The key of the biblatex entry, used as an id in the database as well.
    pub bib_key: String,
    /// The raw biblatex string for a single entry.
    pub biblatex: String,
    pub ctime: DateTime,
    pub utime: DateTime,
}
