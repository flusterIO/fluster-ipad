use conundrum::ecosystem::db::db_traits::db_identifiable::DatabaseIdentifiable;
use conundrum::ecosystem::db::db_traits::entity_crud::EntityCRUD;
use conundrum::ecosystem::db::parameters::general::pagination::PaginationParams;
use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use conundrum::ecosystem::error_handling::server_error::ServerError;
use conundrum_db::vector::models::ecosystem_data::ecosystem_setting_key::setting_key::Setting;
use conundrum_db::vector::models::ecosystem_data::ecosystem_setting_key::unique_setting_key::UniqueSettingKey;
use conundrum_db::vector::models::ecosystem_data::ecosytem_setting_types::ecosystem_setting_model::EcosystemSettingModel;
use conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState;
use rspc::{Procedure, Router};
use std::ops::Index;
use std::sync::Arc;

pub fn get_settings_router() -> Router<Arc<ServerState>> {
    Router::<Arc<ServerState>>::new().procedure("save",
                                            Procedure::<Arc<ServerState>, Setting, ()>::builder::<ServerError>().mutation(|state: Arc<ServerState>, req: Setting| async move {
                                                let db = Arc::clone(&state.db);
                                                req.save(&db).await?;
                                                                                   Ok(())
                                                                               }))
.procedure("read",
                                            Procedure::<Arc<ServerState>, UniqueSettingKey, Option<Setting>>::builder::<ServerError>().query(|state: Arc<ServerState>, req: UniqueSettingKey| async move {
                                                let predicate = req.to_predicate("key");
                                                let models = EcosystemSettingModel::get_by_predicate(Some(predicate), Some(PaginationParams::single()), None, &Arc::clone(&state.db)).await
                                                    .map_err(ServerError::DatabaseError)?;
                                                match models.len() {
                                                    0 => {
                                                        log::warn!("Setting {} not found. This setting may just have not been set yet as this is not seeded with the database.", req);
                                                        Ok(None)
                                                    }
                                                    1 => {
                                                        let item = models.index(0);
                                                        Ok(Some(item.data.clone()))
                                                    }
                                                    _ => {
                                                        log::warn!("Found multiple settings with the {} keu. This ain't good.", req);
                                                        Err(
                                                            ServerError::DatabaseError(DatabaseError::InvalidSetting(req.to_string()))
                                                        )
                                                    }
                                                }
                                                                               }))
}
