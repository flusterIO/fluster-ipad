use conundrum::ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult};
use conundrum_fs::models::user_workspace::workspace_relative_path::{PureWorkspaceRelativePath, WorkspaceRelativePath};
use fake::{Dummy, faker::filesystem::en::FilePath};
use indoc::formatdoc;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;

use crate::vector::{
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
        ai::ai_generated_status::AIGeneratedStatus,
        date_time::date_time::DateTime,
        primitives::{db_id::DatabaseId, id_record::IDRecord},
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue, Dummy)]
pub struct PureCdrmContent {
    pub id: DatabaseId,
    pub title: Option<String>,
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
    async fn save_self(&self, db: &ArcMutexDB) -> DatabaseResult<IDRecord> {
        let db = db.clone().lock_owned().await;
        let r: IDRecord = db.create((Self::table().to_string(), self.id.to_string()))
                            .content(self.clone())
                            .await
                            .map_err(|e| DatabaseError::DatabaseError { source: Some(e) })?
                            .ok_or_else(|| DatabaseError::DatabaseError { source: None })?;
        drop(db);
        Ok(r)
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
        let db = get_database().await.expect("Gets database");
        let res = cdrm.save_self(db)
                      .await
                      .inspect_err(|e| {
                          log::error!("Error: {:?}", e);
                      })
                      .expect("Saves cdrm content without throwing an error.");
        // assert_eq!(result, 4);
    }
}
