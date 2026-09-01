#[macro_export]
macro_rules! crud_router {
    ( $full:ty, $partial:ty ) => {
        {
    rspc::Router::<std::sync::Arc<conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState>>::new()
    .procedure("get_by_predicate", Procedure::<std::sync::Arc<conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState>, conundrum_db::vector::parameters::general::general_query::GeneralQuery, Vec<$full>>::builder::<conundrum::ecosystem::error_handling::server_error::ServerError>().query(|state: std::sync::Arc<ServerState>, params: conundrum_db::vector::parameters::general::general_query::GeneralQuery | async move {
        let r = <$full as conundrum::ecosystem::db::db_traits::entity_crud::EntityCRUD<<$full as conundrum::ecosystem::db::db_traits::db_entity::DBSchema>::PartialUpdateType>>::get_by_predicate(params.predicate, Some(params.pagination), params.sort, std::sync::Arc::clone(&state.db)).await.map_err(|e| {
            log::error!("Error: {:?}", e);
            ServerError::DatabaseError(e)
        })?;
        Ok(r)
        }))
    .procedure("save_many", Procedure::<std::sync::Arc<conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState>, Vec<$full>, ()>::builder::<conundrum::ecosystem::error_handling::server_error::ServerError>().mutation(|state: std::sync::Arc<ServerState>, params: Vec<$full> | async move {
        <$full as conundrum::ecosystem::db::db_traits::entity_crud::EntityCRUD<<$full as conundrum::ecosystem::db::db_traits::db_entity::DBSchema>::PartialUpdateType>>::save_many(params, std::sync::Arc::clone(&state.db)).await.map_err(|e| {
            log::error!("Error: {:?}", e);
            ServerError::DatabaseError(e)
        })?;
        Ok(())
        }))
    .procedure("update_many", Procedure::<std::sync::Arc<conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState>, Vec<<$full as conundrum::ecosystem::db::db_traits::db_entity::DBSchema>::PartialUpdateType>, ()>::builder::<conundrum::ecosystem::error_handling::server_error::ServerError>().mutation(|state: std::sync::Arc<ServerState>, params: Vec<<$full as conundrum::ecosystem::db::db_traits::db_entity::DBSchema>::PartialUpdateType> | async move {
        <$full as conundrum::ecosystem::db::db_traits::entity_crud::EntityCRUD<<$full as conundrum::ecosystem::db::db_traits::db_entity::DBSchema>::PartialUpdateType>>::merge_by_primary_key(params, std::sync::Arc::clone(&state.db)).await.map_err(|e| {
            log::error!("Error: {:?}", e);
            ServerError::DatabaseError(e)
        })?;
        Ok(())
        }))
    .procedure("delete_by_predicate", Procedure::<std::sync::Arc<conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState>, String, ()>::builder::<conundrum::ecosystem::error_handling::server_error::ServerError>().mutation(|state: std::sync::Arc<ServerState>, params: String | async move {
        <$full as conundrum::ecosystem::db::db_traits::entity_crud::EntityCRUD<<$full as conundrum::ecosystem::db::db_traits::db_entity::DBSchema>::PartialUpdateType>>::delete_by_predicate(params.as_str(), std::sync::Arc::clone(&state.db)).await.map_err(|e| {
            log::error!("Error: {:?}", e);
            ServerError::DatabaseError(e)
        })?;
        Ok(())
        }))
        }
    };
}
