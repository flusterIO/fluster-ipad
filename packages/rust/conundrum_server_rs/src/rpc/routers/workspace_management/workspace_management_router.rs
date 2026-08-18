use std::{ops::Index, sync::Arc};

use conundrum::ecosystem::{
    db::{
        db_traits::{async_traits::try_from_async::TryFromAsync, entity_crud::EntityCRUD},
        parameters::general::pagination::PaginationParams,
        tables::DatabaseTable,
    },
    error_handling::db_error::DatabaseError,
};
use conundrum_db::vector::models::{
    ecosystem_data::server_state::server_state::ServerState,
    workspace::{user_workspace::UserWorkspace, user_workspace_count_data::UserWorkspaceCountData},
};
use rspc::{Procedure, Router};

use conundrum::ecosystem::error_handling::server_error::{ServerError, ServerResult};

pub fn get_workspace_management_router() -> Router<Arc<ServerState>> {
    Router::<Arc<ServerState>>::new()
    .procedure("parsable_file_count", Procedure::<Arc<ServerState>, String, UserWorkspaceCountData>::builder::<ServerError>().query(|state: Arc<ServerState>, params: String | async move {
        let predicate = format!("root=\"{}\"", params);
        let wp = UserWorkspace::get_by_predicate(Some(predicate.clone()), Some(PaginationParams::single()), None, &state.db).await.map_err(|e| {
                        ServerError::DatabaseError(e)
                    })?;
        match wp.len() {
           0 => {
               log::error!("Failed querying UserWorkspace");
Err(ServerError::DatabaseError(DatabaseError::FailToQueryEntity { predicate: Some(predicate), table: DatabaseTable::UserWorkspace }))
           },
           1 => {
                    let workspace = wp.index(0);
                    let count_data: UserWorkspaceCountData = UserWorkspaceCountData::try_from_async(workspace.clone()).await.map_err(|e| {
                        ServerError::DatabaseError(e)
                    })?;
                    Ok(count_data)
           },
           _ => {
               log::error!("Duplicate workspaces found");
               Err(
                    ServerError::DatabaseError(DatabaseError::DuplicateEntities)
               )
           }
        }
                                                                               }))
}
