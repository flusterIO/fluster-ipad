use conundrum::ecosystem::{
    db::{db_table_description::DBTableDescription, tables::DatabaseTable},
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use conundrum_db::vector::database::db::ArcMutexDB;
use strum::IntoEnumIterator;

use crate::rpc::server_health::table_health_report::TableHealthReport;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct ServerHealthReport {
    pub table_reports: Vec<TableHealthReport>,
    pub is_healthy: bool,
    pub all_tables_exist: bool,
}

impl ServerHealthReport {
    pub async fn new(_db: &ArcMutexDB) -> DatabaseResult<Self> {
        let db = _db.clone().lock_owned().await;
        let table_names = db.table_names().execute().await.map_err(|e| {
                                                               log::error!("Error: {:?}", e);
                                                               DatabaseError::FailToConnect
                                                           })?;
        let mut table_reports: Vec<TableHealthReport> = Vec::new();
        let mut is_healthy = true;
        let mut all_tables_exist = true;
        for dt in DatabaseTable::iter() {
            let exists = table_names.iter().any(|x| *x == dt.to_string());
            let table_report = TableHealthReport { exists,
                                                   is_temporary_table: dt.is_temporary_vector_table(),
                                                   description: DBTableDescription::from(dt.clone()) };
            table_reports.push(table_report);
            if !exists {
                is_healthy = false;
                all_tables_exist = false;
            }
        }
        Ok(Self { table_reports,
                  is_healthy,
                  all_tables_exist })
    }
}
