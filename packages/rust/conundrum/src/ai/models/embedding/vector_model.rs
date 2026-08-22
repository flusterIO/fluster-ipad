use std::fmt::Display;

use crate::{
    ecosystem::{
        db::{
            db::ArcMutexDB,
            db_traits::{
                db_entity::{DBEntity, DBSchema},
                db_identifiable::DatabaseIdentifiable,
                entity_crud::EntityCRUD,
            },
        },
        error_handling::db_error::DatabaseResult,
    },
    lifted_models::primitives::db_id::DatabaseId,
};

pub trait VectorModel<'a, PrimaryIdType: DatabaseIdentifiable + Display = DatabaseId>:
    DBEntity<'a, PrimaryIdType> + DBSchema<'a> + EntityCRUD<'a, PrimaryIdType, Self::UpdatePartial> {
    type UpdatePartial: DBSchema<'a> + Clone;
    /// Returns (DocumentId, ChunkReferenceKey) in that order.
    ///
    /// For example,
    ///
    /// For
    ///
    /// ```rs
    /// pub struct SomeChunk {
    ///     pub document_id: String,
    ///     pub vector: ...
    /// }
    /// ```
    ///
    /// And the matching document with the id field if `id`, this field should
    /// return `("id", "document_id")`
    fn chunk_field_keys() -> (&'static str, &'static str);
    /// **WARNING**: the default implemntation will only work for chunked
    /// models. This won't work for images or models that are embedded in
    /// one piece.
    async fn clean_related_chunks(&self,
                                  document_identifable_value: PrimaryIdType,
                                  locked_db: &ArcMutexDB)
                                  -> DatabaseResult<()> {
        let (_, id) = Self::chunk_field_keys();
        let predicate = format!("{} = \"{}\"", id, document_identifable_value);
        Self::delete_by_predicate(predicate.as_str(), locked_db).await?;
        Ok(())
    }
}
