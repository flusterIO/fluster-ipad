use std::sync::Arc;

use conundrum::ecosystem::{
    db::tables::DatabaseTable,
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use conundrum_fs::path_utils::ecosystem_paths::get_app_database_dir;
use lancedb::{Table, arrow::arrow_schema::Schema, connect};
use log::warn;

use crate::vector::{
    database::{db::CdrmDb, db_traits::db_entity::DBEntity},
    models::academic::question::flashcard::{flashcard_entity::FlashCardEntity, flashcard_model::FlashCardModel},
};

pub type DatabaseIndexSetupFunction = fn(&CdrmDb) -> DatabaseResult<()>;

struct TableInitData {
    pub table: DatabaseTable,
    pub schema: Arc<Schema>,
    /// An optional function called after the table is created so indices can be
    /// applied.
    pub set_indices: Option<DatabaseIndexSetupFunction>,
}

async fn create_table(db: &lancedb::Connection, schema: &Arc<Schema>, table: &DatabaseTable) -> DatabaseResult<Table> {
    db.create_empty_table(table.to_string(), schema.clone())
      .mode(lancedb::database::CreateTableMode::Create)
      .execute()
      .await
      .map_err(|_| DatabaseError::FailToCreateTable(table.clone()))
}

pub async fn initialize_local_database() -> DatabaseResult<()> {
    let table_data: Vec<TableInitData> = vec![TableInitData { table: DatabaseTable::QAPair,
                                                              schema: FlashCardEntity::arrow_schema(),
                                                              set_indices: None }];
    if let Ok(db_path) = get_app_database_dir() {
        let db = connect(db_path.to_str().unwrap()).execute().await.map_err(|e| {
                                                                        println!("Error in initialize_database: {:?}",
                                                                                 e);
                                                                        DatabaseError::FailToConnect
                                                                    })?;

        for td in table_data.iter() {
            if !td.table.is_temporary_vector_table() {
                if let Ok(res) = create_table(&db, &td.schema, &td.table).await {
                    if let Some(si) = td.set_indices {
                        si(&db)?;
                    }
                } else {
                    let s = td.table.to_model_name();
                    warn!("Conundrum failed while attempting to generate a database table for the `{:?}` model.", s);
                }
            }
        }
    }
    Ok(())
}
