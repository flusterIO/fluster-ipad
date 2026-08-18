use std::sync::Arc;

use crate::{
    routes::fs::route_exists::path_exists,
    rpc::routers::fs::fs_path_simple_result::{FSPathSimpleResult, PathVariant},
};
use conundrum::ecosystem::error_handling::server_error::{ServerError, ServerResult};
use conundrum::{
    ecosystem::{
        db::{db_table_description::DBTableDescription, tables::DatabaseTable},
        error_handling::{conundrum_fs_error::ConundrumFSError, db_error::DatabaseError},
    },
    lang::constants::file_types::ParsableFileType,
};
use conundrum_db::vector::{
    models::ecosystem_data::server_state::server_state::ServerState,
    parameters::fs::path_validity_check::PathValidationRequest,
};
use rspc::{Procedure, Router};

pub fn get_table_router() -> Router<Arc<ServerState>> {
    Router::<Arc<ServerState>>::new()
        .procedure("current_tables",
                                            Procedure::<Arc<ServerState>, (), Vec<DatabaseTable>>::builder::<ServerError>().query(|context: Arc<ServerState>, _: ()| async move {
                                                let db = context.db.clone().lock_owned().await;
                                                let r = db.table_names()
                                                    .execute()
                                                    .await
                                                    .map_err(|e| {
                                                        log::error!("Error retrieving table names: {:?}", e);
                                                        ServerError::DatabaseError(DatabaseError::FailToConnect)
                                                    })?;
                                                let mut database_tables = Vec::new();
                                                for dt in r {
                                                    let t = DatabaseTable::try_from(dt).map_err(|e| {
                                                        ServerError::DatabaseError(e)
                                                    })?;
                                                    database_tables.push(t);
                                                }
                                                Ok(database_tables)
                                                                               }))
        .procedure("describe_table",
                                            Procedure::<Arc<ServerState>, DatabaseTable, DBTableDescription>::builder::<ServerError>().query(|context: Arc<ServerState>, req: DatabaseTable| async move {
                                                let desc = DBTableDescription::from(req);
                                                Ok(desc)
                                                                               }))
}
