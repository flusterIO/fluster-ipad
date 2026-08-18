use crate::ecosystem::db::db_traits::db_entity::DBSchema;

#[macro_export]
macro_rules! impl_default_crud {
    ( $self:ty, $partial:ty, $id_type:ty ) => {
        impl<'a> $crate::ecosystem::db::db_traits::entity_crud::EntityCRUD<'a, $id_type, $partial> for $self {
            async fn get_by_predicate(predicate: Option<String>,
                                      pagination: Option<$crate::ecosystem::db::parameters::general::pagination::PaginationParams>,
                                      sort: Option<Vec<$crate::ecosystem::db::parameters::general::sort_query::SortQuery>>,
                                      db: &$crate::ecosystem::db::db::ArcMutexDB)
                                      -> $crate::ecosystem::error_handling::db_error::DatabaseResult<Vec<Self>>
                where Self: Sized {
                $crate::get_by_predicate!($self, db, predicate, pagination, sort)
            }
        }
    };
}
