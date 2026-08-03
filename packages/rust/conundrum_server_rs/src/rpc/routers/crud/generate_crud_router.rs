#[macro_export]
macro_rules! generate_crud_router {
    ( $self_alias:ty ) => {
        {
    rspc::Router::<RouteContext>::new()
    .procedure("get_many", Procedure::<RouteContext, String, UserWorkspaceCountData>::builder::<ServerError>().query(|_, params: String | async move {
        }
        }
    };
}
