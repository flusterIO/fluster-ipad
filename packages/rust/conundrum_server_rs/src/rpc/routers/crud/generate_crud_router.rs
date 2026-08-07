#[macro_export]
macro_rules! crud_router {
    ( $full:ty, $partial:ty ) => {
        {
    rspc::Router::<$crate::rpc::route_context::RouteContext>::new()
    .procedure("get_by_predicate", Procedure::<$crate::rpc::route_context::RouteContext, PredicateQueryParams, Vec<$full>>::builder::<$crate::errors::server_error::ServerError>().query(|state: RouteContext , params: PredicateQueryParams | async move {
        let r = <$full>::get_by_predicate(params.predicate, params.pagination, &state.db).await.map_err(|e| {
            log::error!("Error: {:?}", e);
            ServerError::DatabaseError(e)
        })?;
        Ok(r)
        }))
    .procedure("save_many", Procedure::<$crate::rpc::route_context::RouteContext, Vec<$full>, ()>::builder::<$crate::errors::server_error::ServerError>().mutation(|state: RouteContext, params: Vec<$full> | async move {
        <$full>::save_many(params, &state.db).await.map_err(|e| {
            log::error!("Error: {:?}", e);
            ServerError::DatabaseError(e)
        })?;
        Ok(())
        }))
    .procedure("update_many", Procedure::<$crate::rpc::route_context::RouteContext, Vec<$partial>, ()>::builder::<$crate::errors::server_error::ServerError>().mutation(|state: RouteContext, params: Vec<$partial> | async move {
        <$full>::merge_by_primary_key(params, &state.db).await.map_err(|e| {
            log::error!("Error: {:?}", e);
            ServerError::DatabaseError(e)
        })?;
        Ok(())
        }))
    .procedure("delete_by_predicate", Procedure::<$crate::rpc::route_context::RouteContext, String, ()>::builder::<$crate::errors::server_error::ServerError>().mutation(|state: RouteContext, params: String | async move {
        <$full>::delete_by_predicate(params.as_str(), &state.db).await.map_err(|e| {
            log::error!("Error: {:?}", e);
            ServerError::DatabaseError(e)
        })?;
        Ok(())
        }))
        }
    };
}
