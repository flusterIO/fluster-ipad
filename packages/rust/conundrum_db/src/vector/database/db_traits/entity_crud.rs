use std::{fmt::Display, sync::Arc};

use arrow_array::{RecordBatch, RecordBatchIterator};
use arrow_schema::ArrowError;
use conundrum::ecosystem::{
    db::{
        tables::DatabaseTable,
        traits::db_entity::{DBEntity, DBSchema},
    },
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use futures_util::TryStreamExt;
use lancedb::query::{ExecutableQuery, QueryBase};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_arrow::{from_record_batch, to_record_batch};

use crate::vector::{
    database::{
        db::ArcMutexDB, db_traits::db_identifiable::DatabaseIdentifiable, open_table::open_table,
        pagination::PaginationParams,
    },
    parameters::general::sort_query::SortQuery,
};

pub trait EntityCRUD<'a, IDType: DatabaseIdentifiable, UpdatePartial: DBSchema<'a> + Clone + Serialize>:
    DBEntity<'a, IDType> + Clone + Serialize {
    async fn save_many(items: Vec<Self>, db: &ArcMutexDB) -> DatabaseResult<()>
        where Self: Sized {
        let schema = Self::schema().map(Arc::new)?;
        let _db = db.clone().lock_owned().await;
        let table = Self::table();
        let tbl = open_table(_db, &table).await.inspect_err(|e| {
                                                    log::error!("Table Error: {:?}", e);
                                                })?;
        let batches = Self::get_record_batch(items.clone()).inspect_err(|e| {
                                                               log::error!("get_record_batch Error: {:?}", e);
                                                           })?;
        let stream = Box::new(RecordBatchIterator::new(vec![batches].into_iter().map(Ok), schema.clone()));
        let primary_key: &[&str] = Self::merge_keys();
        tbl.merge_insert(primary_key)
           .when_matched_update_all(None)
           .when_not_matched_insert_all()
           .clone()
           .execute(stream)
           .await
           .map_err(|e| {
               log::error!("Database Error: {:?}", e);
               DatabaseError::FailToCreateEntity(Self::table().to_model_name())
           })?;
        log::info!("Successfully saved {} `{}` models", items.len(), table.to_model_name());
        Ok(())
    }
    async fn save_one(item: Self, db: &ArcMutexDB) -> DatabaseResult<()>
        where Self: Sized {
        Self::save_many(vec![item], db).await
    }

    async fn get_by_predicate(predicate: Option<String>,
                              pagination: Option<PaginationParams>,
                              sort: Option<Vec<SortQuery>>,
                              db: &ArcMutexDB)
                              -> DatabaseResult<Vec<Self>>
        where Self: Sized {
        // crate::get_by_predicate
        todo!()
    }

    async fn delete_by_predicate<'b>(predicate: &'b str, db: &ArcMutexDB) -> DatabaseResult<()> {
        let tbl = Self::table();
        let _db = db.clone().lock_owned().await;
        let db_tbl = open_table(_db, &tbl).await?;
        // let pk = Self::primary_key();
        db_tbl.delete(predicate).await.map_err(|e| {
                                           log::error!("Error: {:?}", e);
                                           DatabaseError::FailToDelete(tbl.clone())
                                       })?;
        log::info!("Successfully delete `{}` models by the predicate `{}`.", tbl.to_model_name(), predicate);
        Ok(())
    }
    async fn delete_by_primary_key(id: IDType, db: &ArcMutexDB) -> DatabaseResult<()> {
        Self::delete_by_predicate(id.to_predicate(Self::primary_key()).as_str(), db).await
    }
    async fn merge_by_primary_key(items: Vec<UpdatePartial>, db: &ArcMutexDB) -> DatabaseResult<()> {
        let tbl = Self::table();
        let _db = db.clone().lock_owned().await;
        let db_tbl = open_table(_db, &tbl).await?;
        let merge_keys = Self::merge_keys();

        let partial_fields = UpdatePartial::arrow_fields()?;
        let record_batch = to_record_batch(&partial_fields, &items.clone()).map_err(|e| {
                                                                               log::error!("Error: {:?}", e);
                                                                               DatabaseError::SerializationError
                                                                           })?;

        let schema = Self::schema().map(Arc::new)?;
        let stream = Box::new(RecordBatchIterator::new(vec![Ok(record_batch)].into_iter(), schema.clone()));
        db_tbl.merge_insert(merge_keys)
              .when_matched_update_all(None)
              .when_not_matched_insert_all()
              .clone()
              .execute(stream)
              .await
              .map_err(|e| {
                  println!("Error: {:?}", e);
                  DatabaseError::SerializationError
              })?;
        log::info!("Successfully merged `{}` models.", tbl.to_model_name());
        Ok(())
    }
}
