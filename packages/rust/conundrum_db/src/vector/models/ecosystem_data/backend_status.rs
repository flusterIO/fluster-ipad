use std::sync::Arc;

use conundrum::ecosystem::db::{
    db_traits::async_traits::try_from_async::{FromAsync, TryFromAsync},
    tables::DatabaseTable,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::vector::models::ecosystem_data::server_state::server_state::ServerState;

/// Just like the ServerStatus, but minimal, where as that is meant to provide
/// enough information to provide the user with a full UI, this one will not
/// grow beyond this.
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
pub struct BackendStatus {
    pub local_client_access: bool,
    pub remote_client_access: bool,
    pub all_tables_exist: bool,
}

impl FromAsync<Arc<ServerState>> for BackendStatus {
    async fn from_async(input: Arc<ServerState>) -> Self
        where Self: Sized {
        let db = input.db.clone().lock_owned().await;
        for k in DatabaseTable::iter() {
            if db.open_table(k.to_string()).execute().await.is_err() {
                return Self { local_client_access: input.local_client.is_some(),
                              remote_client_access: input.remote_client.is_some(),
                              all_tables_exist: false };
            }
        }
        Self { local_client_access: input.local_client.is_some(),
               remote_client_access: input.remote_client.is_some(),
               all_tables_exist: true }
    }
}
