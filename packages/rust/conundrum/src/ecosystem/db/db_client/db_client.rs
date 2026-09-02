use std::sync::Arc;

use crate::{
    ecosystem::{
        db::{
            db_client::db_client_trait::DBClient as DBClientTrait, helpers::open_table::open_table,
            tables::DatabaseTable,
        },
        error_handling::db_error::{DatabaseError, DatabaseResult},
    },
    lang::lib::shared::utility_types::ArcTokioMutex,
};
use lancedb::{Connection, Table};
use serde::{Deserialize, Serialize};

// use crate::vector::database::db_client::db_client_trait::DBClient;

#[derive(Clone)]
pub struct DBClient(pub ArcTokioMutex<Connection>);

impl DBClient {
    pub async fn get_table(&self, table: &DatabaseTable) -> DatabaseResult<Table> {
        let db = self.0.clone().lock_owned().await;
        let tbl = open_table(db, table.clone()).await?;
        Ok(tbl)
    }

    pub fn inner_clone(&self) -> ArcTokioMutex<Connection> {
        self.0.clone()
    }

    pub fn inner_arc(&self) -> ArcTokioMutex<Connection> {
        Arc::clone(&self.0)
    }
}

impl DBClientTrait for DBClient {
    async fn clear_table(&self, table: DatabaseTable) -> DatabaseResult<()> {
        let tbl = self.get_table(&table).await?;
        let _ = tbl.delete("1 = 1").await.map_err(|e| {
                                             log::error!("Database Error: {:#?}", e);
                                             DatabaseError::FailToDelete(table.clone())
                                         });
        Ok(())
    }
}
