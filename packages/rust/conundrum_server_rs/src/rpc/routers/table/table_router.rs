use crate::{
    errors::server_error::{ServerError, ServerResult},
    routes::fs::route_exists::path_exists,
    rpc::{
        route_context::RouteContext,
        routers::fs::fs_path_simple_result::{FSPathSimpleResult, PathVariant},
    },
};
use conundrum::{
    ecosystem::{
        db::{db_table_description::DBTableDescription, tables::DatabaseTable},
        error_handling::{conundrum_fs_error::ConundrumFSError, db_error::DatabaseError},
    },
    lang::constants::file_types::ParsableFileType,
};
use conundrum_db::vector::{
    database::db_traits::async_traits::actionable_request::ActionableRequest,
    parameters::fs::path_validity_check::PathValidationRequest,
};
use rspc::{Procedure, Router};

pub fn get_table_router() -> Router<RouteContext> {
    Router::<RouteContext>::new()
        .procedure("current_tables",
                                            Procedure::<RouteContext, (), Vec<DatabaseTable>>::builder::<ServerError>().query(|context: RouteContext, _: ()| async move {
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
                                            Procedure::<RouteContext, DatabaseTable, DBTableDescription>::builder::<ServerError>().query(|context: RouteContext, req: DatabaseTable| async move {
                                                let desc = DBTableDescription::from(req);
                                                Ok(desc)
                                                                               }))
}
