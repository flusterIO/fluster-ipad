use std::sync::Arc;

use conundrum::{
    ecosystem::{
        db::db_traits::{db_entity::DBSchema, entity_crud::EntityCRUD},
        error_handling::{db_error::DatabaseError, server_error::ServerError},
    },
    lang::lib::std_lib_impls::json_string::QuotedString,
};
use conundrum_db::vector::models::{
    ecosystem_data::server_state::server_state::ServerState,
    workspace::{
        sync::{sync_context::SyncContext, sync_workspace::sync_registered_workspace},
        user_workspace::UserWorkspace,
    },
};
use rspc::{Procedure, Router};

pub fn get_sync_router() -> Router<Arc<ServerState>> {
    Router::<Arc<ServerState>>::new().procedure(
        "workspace",
        Procedure::<Arc<ServerState>, String, SyncContext>::builder::<ServerError>()
            .mutation(|state: Arc<ServerState>, root: String| async move {
                let predicate = format!("root = {}", root.to_quoted_string()?);
                let workspace =
                    <UserWorkspace as EntityCRUD<<UserWorkspace as DBSchema>::PartialUpdateType>>::get_one_by_predicate(
                        Some(predicate),
                        None,
                        state.db.clone(),
                    )
                    .await?
                    .ok_or_else(|| ServerError::DatabaseError(DatabaseError::InvalidWorkspacePath(root.clone())))?;

                sync_registered_workspace(state.db.clone(), workspace)
                    .await
                    .map_err(ServerError::DatabaseError)
            }),
    )
}
