use conundrum::ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult};
use conundrum_fs::models::user_workspace::workspace_relative_path::PureWorkspaceRelativePath;
use fake::Dummy;
use indoc::formatdoc;
use serde::{Deserialize, Serialize};
use surrealdb_types::{RecordId, SurrealValue, Value};

use crate::{
    test_utils::faker_generators::fake_database_id::fake_database_id,
    vector::{
        database::{
            db::ArcMutexDB,
            db_traits::{
                database_field::DatabaseField, pure_model_instance::PureModelInstanceMethods,
                pure_model_static::PureModelStaticMethods,
            },
            primitive_field_schema_generators::string_field_def_generator::{
                optional_string_field_definition, string_field_definition,
            },
        },
        models::{
            ai::ai_generated_status::AIGeneratedStatus, date_time::date_time::DateTime, primitives::db_id::DatabaseId,
        },
    },
};

use crate::test_utils::faker_generators::fake_cdrm_content::fake_cdrm_content;

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue, Dummy)]
pub struct PureCdrmContent {
    #[surreal(flatten)]
    #[dummy(expr = "fake_database_id(\"cdrm\")")]
    pub id: DatabaseId,
    pub title: Option<String>,
    #[dummy(expr = "fake_cdrm_content(0..100)")]
    pub content: String,
    pub ai_generated: AIGeneratedStatus,
    /// The path relative to the root of the user's 'library'.
    pub ws_path: Option<PureWorkspaceRelativePath>,
    pub ctime: DateTime,
    pub utime: DateTime,
}

impl PureModelStaticMethods for PureCdrmContent {
    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        conundrum::ecosystem::db::tables::DatabaseTable::Cdrm
    }

    fn schema() -> String {
        let tbl = Self::table();
        formatdoc! {"
        {}
        {}
        {}
        {}
        {}
        {}
        {}
        ", 
            DatabaseId::field_definition("id", &tbl),
            optional_string_field_definition("title", &tbl),
            string_field_definition("content", &tbl),
            AIGeneratedStatus::field_definition("ai_generated", &tbl),
            optional_string_field_definition("ws_path", &tbl),
            DateTime::field_definition("ctime", &tbl),
            DateTime::field_definition("utime", &tbl),
        }
    }
}

impl PureModelInstanceMethods for PureCdrmContent {
    async fn upsert_self(&self, db: &ArcMutexDB) -> DatabaseResult<RecordId> {
        let db = db.clone().lock_owned().await;
        let value = db.create((Self::table().to_string(), self.id.to_string()))
                      .content(self.clone())
                      .await
                      .map_err(|e| DatabaseError::DatabaseError { source: Some(e) })?
                      .ok_or(DatabaseError::DatabaseError { source: None })?;
        drop(db);
        let deserialized = RecordId::from_value(value).map_err(|e| {
                                                          log::error!("Error: {:?}", e);
                                                          DatabaseError::SerializationError
                                                      })?;
        Ok(deserialized)
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};

    use crate::vector::database::db::get_database;

    use super::*;

    #[tokio::test]
    async fn saves_conundrum_content() {
        let cdrm: PureCdrmContent = Faker.fake();
        dbg!(cdrm.clone());
        let db = get_database().await.expect("Gets database");
        let res = cdrm.upsert_self(db)
                      .await
                      .inspect_err(|e| {
                          log::error!("Error: {:?}", e);
                      })
                      .expect("Saves cdrm content without throwing an error.");
        println!("{:#?}", res);
    }
}
