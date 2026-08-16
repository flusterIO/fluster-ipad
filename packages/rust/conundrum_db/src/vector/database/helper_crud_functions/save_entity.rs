use std::{fmt::Debug, sync::Arc};

use arrow_array::RecordBatchIterator;
use conundrum::ecosystem::{
    db::traits::db_entity::DBEntity,
    error_handling::db_error::{DatabaseError, DatabaseResult},
};

use crate::vector::database::{db::ArcMutexDB, open_table::open_table};

pub async fn save_entities<'a, T, IDType>(items: Vec<T>, db: ArcMutexDB) -> DatabaseResult<()>
    where T: DBEntity<'a, IDType> + Clone + Debug {
    let schema = T::schema().map(Arc::new)?;
    let _db = db.clone().lock_owned().await;
    let table = T::table();
    let tbl = open_table(_db, &table).await.inspect_err(|e| {
                                                log::error!("Table Error: {:?}", e);
                                            })?;
    let batches = T::get_record_batch(items.clone()).inspect_err(|e| {
                                                        log::error!("get_record_batch Error: {:?}", e);
                                                    })?;
    let stream = Box::new(RecordBatchIterator::new(vec![batches].into_iter().map(Ok), schema.clone()));
    let primary_key: &[&str] = T::merge_keys();
    tbl.merge_insert(primary_key)
       .when_matched_update_all(None)
       .when_not_matched_insert_all()
       .clone()
       .execute(stream)
       .await
       .map_err(|e| {
           log::error!("Database Error: {:?}", e);
           DatabaseError::FailToCreateEntity(T::table().to_model_name())
       })?;
    log::info!("Successfully saved {} `{}` models", items.len(), table.to_model_name());
    Ok(())
}
