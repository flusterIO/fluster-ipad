#[macro_export]
macro_rules! get_by_predicate {
    ( $self:ty, $db:ident, $predicate:ident, $pagination:ident ) => {{
        use futures_util::TryStreamExt;
        use lancedb::query::ExecutableQuery;
        use lancedb::query::QueryBase;
        let _db = $db.clone().lock_owned().await;
        let self_table = Self::table();
        let tbl = $crate::vector::database::open_table::open_table(_db, &self_table).await?;
        let mut query_builder = tbl.query();
        if let Some(_predicate) = $predicate.clone() {
            query_builder = query_builder.only_if(_predicate);
        }
        if let Some(_pagination) = $pagination {
            let (limit, offset) = _pagination.to_limit_and_offset();
            query_builder = query_builder.limit(limit).offset(offset);
        }
        let res = query_builder.execute()
                               .await
                               .map_err(|e| {
                                   log::error!("Error: {:?}", e);
                                   conundrum::ecosystem::error_handling::db_error::DatabaseError::FailToQueryEntity { predicate: $predicate.clone(),
                                                                      table: self_table.clone() }
                               })?
                               .try_collect::<Vec<_>>()
                               .await
                               .map_err(|e| {
                                   log::error!("Error: {:?}", e);
                                   conundrum::ecosystem::error_handling::db_error::DatabaseError::SerializationError
                               })?;

        if res.is_empty() {
            return Ok(Vec::new());
        }

        let mut items: Vec<$self> = Vec::new();

        for record_batch in res.iter() {
            let r: Vec<$self> = serde_arrow::from_record_batch(record_batch).map_err(|e| {
                                    log::error!("Error: {:?}", e);
                                    conundrum::ecosystem::error_handling::db_error::DatabaseError::SerializationError
                                })?;
            items.extend(r);
        }

        Ok(items)
    }};
}
