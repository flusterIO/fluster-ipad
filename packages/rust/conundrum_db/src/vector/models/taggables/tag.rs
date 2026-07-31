use conundrum::ecosystem::{db::tables::DatabaseTable, error_handling::db_error::DatabaseError};
use fake::Dummy;
use indoc::formatdoc;
use serde::{Deserialize, Serialize};
use specta::Type;
use surrealdb::types::SurrealValue;
use surrealdb_types::RecordId;

use crate::{
    test_utils::faker_generators::fake_words_as_string::fake_words_as_string,
    vector::{
        database::db_traits::{
            database_field::DatabaseField, pure_model_instance::PureModelInstanceMethods,
            pure_model_static::PureModelStaticMethods,
        },
        models::{
            date_time::date_time::DateTime,
            primitives::{case_insensitive_string::CaseInsensitiveString, db_id::DatabaseId},
            taggables::tag_location::TagLocation,
        },
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue, Dummy, Type)]
pub struct Tag {
    pub id: DatabaseId,
    #[dummy(faker = "fake_words_as_string(0..10)")]
    pub value: CaseInsensitiveString,
    pub location: TagLocation,
    pub ctime: DateTime,
}

impl Into<RecordId> for Tag {
    fn into(self) -> RecordId {
        self.id.to_record_id()
    }
}

impl PureModelStaticMethods for Tag {
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
        DatabaseTable::Tag
    }
}

impl From<String> for Tag {
    /// WARNING: Calling this method will automatically apply the location of
    /// 'body', making this tag expendable when it's time to sync the
    /// datbase with the filesystem again.
    fn from(value: String) -> Self {
        Tag { id: DatabaseId::new(Self::table()),
              value: CaseInsensitiveString::from(value),
              location: TagLocation::Body,
              ctime: DateTime::new_now() }
    }
}

impl PureModelInstanceMethods for Tag {
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
    async fn saves_test_tag() {
        let test_tag: Tag = Faker.fake();
        let db = get_database().await.expect("Gets database without throwing an error.");
        let res = test_tag.upsert_self(db)
                          .await
                          .inspect_err(|e| {
                              log::error!("Error: {:?}", e);
                          })
                          .expect("Saves test tag without throwing an error;");
        // assert_eq!(result, 4);
    }
}
