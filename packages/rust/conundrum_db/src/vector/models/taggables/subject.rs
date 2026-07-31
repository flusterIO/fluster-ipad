use fake::Dummy;
use indoc::formatdoc;
use serde::{Deserialize, Serialize};
use specta::Type;
use surrealdb::types::SurrealValue;
use surrealdb_types::RecordId;

use crate::vector::{
    database::db_traits::{
        database_field::DatabaseField, pure_model_instance::PureModelInstanceMethods,
        pure_model_static::PureModelStaticMethods,
    },
    models::{
        date_time::date_time::DateTime,
        primitives::{case_insensitive_string::CaseInsensitiveString, db_id::DatabaseId},
        taggables::tag_location::TagLocation,
    },
};

use conundrum::ecosystem::{db::tables::DatabaseTable, error_handling::db_error::DatabaseError};

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue, Dummy, Type)]
pub struct Subject {
    pub id: DatabaseId,
    pub value: CaseInsensitiveString,
    pub location: TagLocation,
    pub ctime: DateTime,
}

impl Into<RecordId> for Subject {
    fn into(self) -> RecordId {
        self.id.to_record_id()
    }
}

impl From<String> for Subject {
    fn from(value: String) -> Self {
        Subject { id: DatabaseId::new(Self::table()),
                  value: CaseInsensitiveString::from(value),
                  location: TagLocation::Body,
                  ctime: DateTime::new_now() }
    }
}

impl PureModelStaticMethods for Subject {
    fn schema() -> String {
        let tbl = Self::table();
        formatdoc! {"
        {};
        {};
        {};
        {};
        ", DatabaseId::field_definition("id", &tbl), CaseInsensitiveString::field_definition("value", &tbl), TagLocation::field_definition("location", &tbl), DateTime::field_definition("ctime", &tbl)}
    }

    fn table() -> DatabaseTable {
        DatabaseTable::Subject
    }
}

impl PureModelInstanceMethods for Subject {
    async fn upsert_self(
        &self,
        db: &crate::vector::database::db::ArcMutexDB)
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<surrealdb_types::RecordId> {
        let locked_db = db.clone().lock_owned().await;
        let res: Option<RecordId> = locked_db.upsert(self.id.to_record_id())
                                             .content(self.clone())
                                             .await
                                             .map_err(|e| DatabaseError::DatabaseError { source: Some(e) })?;
        drop(locked_db);
        match res {
            Some(s) => Ok(s),
            None => Err(DatabaseError::DatabaseError { source: None }),
        }
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};

    use crate::vector::database::db::get_database;

    use super::*;

    #[tokio::test]
    async fn saves_test_subject() {
        let test_subject: Subject = Faker.fake();
        let db = get_database().await.expect("Gets database without throwing an error.");
        let res = test_subject.upsert_self(db)
                              .await
                              .inspect_err(|e| {
                                  log::error!("Error: {:?}", e);
                              })
                              .expect("Saves test subject without throwing an error;");
        // assert_eq!(result, 4);
    }
}
