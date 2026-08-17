#[macro_export]
macro_rules! crud_router {
    ( $full:ty, $partial:ty ) => {
        {
    rspc::Router::<std::sync::Arc<conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState>>::new()
    .procedure("get_by_predicate", Procedure::<std::sync::Arc<conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState>, conundrum_db::vector::parameters::general::general_query::GeneralQuery, Vec<$full>>::builder::<$crate::errors::server_error::ServerError>().query(|state: std::sync::Arc<ServerState>, params: conundrum_db::vector::parameters::general::general_query::GeneralQuery | async move {
        let r = <$full>::get_by_predicate(params.predicate, Some(params.pagination), params.sort, &state.db).await.map_err(|e| {
            log::error!("Error: {:?}", e);
            ServerError::DatabaseError(e)
        })?;
        Ok(r)
        }))
    .procedure("save_many", Procedure::<std::sync::Arc<conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState>, Vec<$full>, ()>::builder::<$crate::errors::server_error::ServerError>().mutation(|state: std::sync::Arc<ServerState>, params: Vec<$full> | async move {
        <$full>::save_many(params, &state.db).await.map_err(|e| {
            log::error!("Error: {:?}", e);
            ServerError::DatabaseError(e)
        })?;
        Ok(())
        }))
    .procedure("update_many", Procedure::<std::sync::Arc<conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState>, Vec<$partial>, ()>::builder::<$crate::errors::server_error::ServerError>().mutation(|state: std::sync::Arc<ServerState>, params: Vec<$partial> | async move {
        <$full>::merge_by_primary_key(params, &state.db).await.map_err(|e| {
            log::error!("Error: {:?}", e);
            ServerError::DatabaseError(e)
        })?;
        Ok(())
        }))
    .procedure("delete_by_predicate", Procedure::<std::sync::Arc<conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState>, String, ()>::builder::<$crate::errors::server_error::ServerError>().mutation(|state: std::sync::Arc<ServerState>, params: String | async move {
        <$full>::delete_by_predicate(params.as_str(), &state.db).await.map_err(|e| {
            log::error!("Error: {:?}", e);
            ServerError::DatabaseError(e)
        })?;
        Ok(())
        }))
        }
    };
}
