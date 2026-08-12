use crate::{errors::server_error::ServerError, rpc::route_context::RouteContext};
use conundrum::ecosystem::db::{db_table_description::DBTableDescription, tables::DatabaseTable};
use rspc::{Procedure, Router};
use strum::IntoEnumIterator;

pub fn get_describe_router() -> Router<RouteContext> {
    Router::<RouteContext>::new()
        .procedure("table",
                                            Procedure::<RouteContext, DatabaseTable, DBTableDescription>::builder::<ServerError>().query(|_, req: DatabaseTable| async move {
                                                let desc: DBTableDescription = DBTableDescription::from(req);
                                                                                   Ok(desc)
                                                                               }))
        .procedure("all_tables",
                                            Procedure::<RouteContext, (), Vec<DBTableDescription>>::builder::<ServerError>().query(|_, _: ()| async move {
                                                let items = DatabaseTable::iter()
                                                    .map(DBTableDescription::from)
                                                    .collect::<Vec<DBTableDescription>>();
                                                Ok(items)
                                                                               }))
}
