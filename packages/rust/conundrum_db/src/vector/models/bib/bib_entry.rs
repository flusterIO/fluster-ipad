use conundrum::{
    bibliography::{bib_entry::BibEntry, split_bibtex_by_entries::split_biblatex_to_raw_strings},
    ecosystem::{
        db::tables::DatabaseTable,
        error_handling::db_error::{DatabaseError, DatabaseResult},
    },
};
use indoc::formatdoc;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;
use surrealdb_types::RecordId;

use crate::vector::{
    database::{
        db_traits::{
            database_field::DatabaseField, pure_model_instance::PureModelInstanceMethods,
            pure_model_static::PureModelStaticMethods,
        },
        primitive_field_schema_generators::string_field_def_generator::{
            boolean_field_definition, string_field_definition, unique_string_field_definition,
        },
    },
    models::{date_time::date_time::DateTime, primitives::db_id::DatabaseId},
};

#[derive(Serialize, Deserialize, Clone, SurrealValue, Debug)]
pub struct BibEntryModel {
    pub id: DatabaseId,
    /// The key of the biblatex entry, used as an id in the database as well.
    pub key: String,
    /// The raw biblatex string for a single entry.
    pub biblatex: String,
    /// A boolean indicating if this literature was already reviewed by the
    /// user.
    pub read: bool,
    pub ctime: DateTime,
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
                           Ok(BibEntryModel { id: DatabaseId::new(Self::table()),
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

impl PureModelStaticMethods for BibEntryModel {
    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::BibEntry
    }

    fn schema() -> String {
        let tbl = Self::table();
        formatdoc! {"
        {};
        {};
        {};
        {};
        {};
        {};
            ", DatabaseId::field_definition("id", &tbl),
            unique_string_field_definition("key", &tbl),
            string_field_definition("biblatex", &tbl),
            boolean_field_definition("read", &tbl),
            DateTime::field_definition("ctime", &tbl),
            DateTime::field_definition("utime", &tbl),
        }
    }
}

impl PureModelInstanceMethods for BibEntryModel {
    async fn upsert_self(&self,
                         db: &crate::vector::database::db::ArcMutexDB)
                         -> DatabaseResult<surrealdb_types::RecordId> {
        let locked_db = db.clone().lock_owned().await;
        let r: RecordId = locked_db.upsert(self.id.to_record_id())
                                   .content(self.clone())
                                   .await
                                   .map_err(|e| DatabaseError::DatabaseError { source: Some(e) })?
                                   .ok_or(DatabaseError::DatabaseError { source: None })?;
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::database::db::get_database;

    use super::*;

    #[tokio::test]
    async fn saves_bib_entries() {
        let biblatex = include_str!("../../../../tests/sample_bib.bib");
        let entries = BibEntryModel::from_biblatex(biblatex).expect("Parses biblatex correctly.");
        let db = get_database().await.expect("Gets database.");
        for entry in entries {
            entry.upsert_self(db).await.expect("Saves bibliography entry without throwing an error.");
        }
    }
}
