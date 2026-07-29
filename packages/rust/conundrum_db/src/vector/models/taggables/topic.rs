use conundrum::ecosystem::{db::tables::DatabaseTable, error_handling::db_error::DatabaseError};
use fake::Dummy;
use indoc::formatdoc;
use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;
use surrealdb_types::RecordId;

use crate::vector::{
    database::db_traits::{
        database_field::DatabaseField, pure_model_instance::PureModelInstanceMethods,
        pure_model_static::PureModelStaticMethods,
    },
    models::{
        date_time::date_time::DateTime, primitives::case_insensitive_string::CaseInsensitiveString,
        taggables::tag_location::TagLocation,
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue, Dummy)]
pub struct Topic {
    pub value: CaseInsensitiveString,
    pub location: TagLocation,
    pub ctime: DateTime,
}

impl PureModelStaticMethods for Topic {
    fn schema() -> String {
        let tbl = Self::table();
        formatdoc! {"
        {}
        {}
        {}
        ", CaseInsensitiveString::field_definition("value", &tbl), TagLocation::field_definition("location", &tbl), DateTime::field_definition("ctime", &tbl)}
    }

    fn table() -> DatabaseTable {
        DatabaseTable::Topic
    }
}

impl From<String> for Topic {
    fn from(value: String) -> Self {
        Topic { value: CaseInsensitiveString::from(value),
                location: TagLocation::FrontMatter,
                ctime: DateTime::new_now() }
    }
}

impl PureModelInstanceMethods for Topic {
    async fn upsert_self(
        &self,
        db: &crate::vector::database::db::ArcMutexDB)
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<surrealdb_types::RecordId> {
        let locked_db = db.clone().lock_owned().await;
        let res: Option<RecordId> = locked_db.upsert((DatabaseTable::Tag.to_string(),
                                                      self.value.to_comparison_string()))
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
    async fn saves_test_topic() {
        let test_topic: Topic = Faker.fake();
        let db = get_database().await.expect("Gets database without throwing an error.");
        let res = test_topic.upsert_self(db)
                            .await
                            .inspect_err(|e| {
                                log::error!("Error: {:?}", e);
                            })
                            .expect("Saves test topic without throwing an error;");
    }
}
