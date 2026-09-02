use conundrum::ecosystem::db::db_traits::db_identifiable::DatabaseIdentifiable;
use conundrum::ecosystem::db::db_traits::entity_crud::EntityCRUD;
use conundrum::ecosystem::db::parameters::general::pagination::PaginationParams;
use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use conundrum::ecosystem::error_handling::server_error::ServerError;
use conundrum_db::vector::models::ecosystem_data::ecosystem_setting_key::setting_key::Setting;
use conundrum_db::vector::models::ecosystem_data::ecosystem_setting_key::unique_setting_key::UniqueSettingKey;
use conundrum_db::vector::models::ecosystem_data::ecosytem_setting_types::ecosystem_setting_model::EcosystemSettingModel;
use conundrum_db::vector::models::ecosystem_data::ecosytem_setting_types::stringified_setting::EcosystemSettingEntity;
use conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState;
use rspc::{Procedure, Router};
use std::ops::Index;
use std::sync::Arc;

pub fn get_settings_router() -> Router<Arc<ServerState>> {
    Router::<Arc<ServerState>>::new().procedure("save",
                                            Procedure::<Arc<ServerState>, Setting, ()>::builder::<ServerError>().mutation(|state: Arc<ServerState>, req: Setting| async move {
                                                req.save(state.db.clone()).await?;
                                                                                   Ok(())
                                                                               }))
.procedure("read",
                                            Procedure::<Arc<ServerState>, UniqueSettingKey, Setting>::builder::<ServerError>().query(|state: Arc<ServerState>, req: UniqueSettingKey| async move {
                                                let predicate = req.to_predicate("key");
                                                let value = req.read_setting(state.db.clone()).await?;
                                                Ok(value.data)
                                                                               }))
}
