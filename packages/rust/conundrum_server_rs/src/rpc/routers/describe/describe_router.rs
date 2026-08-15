use std::sync::Arc;

use crate::errors::server_error::ServerError;
use crate::server_state::ServerState;
use conundrum::ecosystem::db::{db_table_description::DBTableDescription, tables::DatabaseTable};
use rspc::{Procedure, Router};
use strum::IntoEnumIterator;

pub fn get_describe_router() -> Router<Arc<ServerState>> {
    Router::<Arc<ServerState>>::new()
        .procedure("table",
                                            Procedure::<Arc<ServerState>, DatabaseTable, DBTableDescription>::builder::<ServerError>().query(|_, req: DatabaseTable| async move {
                                                let desc: DBTableDescription = DBTableDescription::from(req);
                                                                                   Ok(desc)
                                                                               }))
        .procedure("all_tables",
                                            Procedure::<Arc<ServerState>, (), Vec<DBTableDescription>>::builder::<ServerError>().query(|_, _: ()| async move {
                                                let items = DatabaseTable::iter()
                                                    .map(DBTableDescription::from)
                                                    .collect::<Vec<DBTableDescription>>();
                                                Ok(items)
                                                                               }))
}
