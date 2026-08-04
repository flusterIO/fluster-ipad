use std::fmt::Display;

use arrow_array::{RecordBatch, RecordBatchIterator};
use arrow_schema::ArrowError;
use conundrum::ecosystem::{
    db::tables::DatabaseTable,
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use futures_util::TryStreamExt;
use lancedb::query::{ExecutableQuery, QueryBase};
use serde::{Serialize, de::DeserializeOwned};
use serde_arrow::{from_record_batch, to_record_batch};

use crate::vector::database::{
    db::ArcMutexDB,
    db_traits::{
        db_entity::{ArrowSchemaRepresentable, DBEntity},
        db_identifiable::DatabaseIdentifiable,
    },
    open_table::open_table,
    pagination::PaginationParams,
};

pub trait EntityCRUD<IDType: DatabaseIdentifiable, UpdatePartial: ArrowSchemaRepresentable + Clone + Serialize>:
    DBEntity + Clone + Serialize {
    async fn save_many(items: Vec<Self>, db: &ArcMutexDB) -> DatabaseResult<()>
        where Self: Sized {
        let schema = Self::arrow_schema();
        let _db = db.clone().lock_owned().await;
        let tbl = open_table(_db, &Self::table()).await?;
        let batches = Self::get_record_batch(items)?;
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
        Ok(())
    }
    async fn save_one(item: Self, db: &ArcMutexDB) -> DatabaseResult<()>
        where Self: Sized {
        Self::save_many(vec![item], db).await
    }

    async fn get_by_predicate<'a>(predicate: Option<String>,
                                  pagination: Option<PaginationParams>,
                                  db: &ArcMutexDB)
                                  -> DatabaseResult<Vec<Self>>
        where Self: Sized + DeserializeOwned {
        let _db = db.clone().lock_owned().await;
        let self_table = Self::table();
        let tbl = open_table(_db, &self_table).await?;
        let mut query_builder = tbl.query();
        if let Some(_predicate) = predicate.clone() {
            query_builder = query_builder.only_if(_predicate);
        }
        if let Some(_pagination) = pagination {
            let (limit, offset) = _pagination.to_limit_and_offset();
            query_builder = query_builder.limit(limit).offset(offset);
        }
        let res = query_builder.execute()
                               .await
                               .map_err(|e| {
                                   log::error!("Error: {:?}", e);
                                   DatabaseError::FailToQueryEntity { predicate: predicate.clone(),
                                                                      table: self_table.clone() }
                               })?
                               .try_collect::<Vec<_>>()
                               .await
                               .map_err(|e| {
                                   log::error!("Error: {:?}", e);
                                   DatabaseError::SerializationError
                               })?;

        let mut items: Vec<Self> = Vec::new();

        for record_batch in res {
            let r: Vec<Self> = from_record_batch(&record_batch).map_err(|e| {
                                                                   log::error!("Error: {:?}", e);
                                                                   DatabaseError::SerializationError
                                                               })?;
            items.extend(r);
        }

        Ok(items)
    }

    async fn delete_by_predicate<'a>(predicate: &'a str, db: &ArcMutexDB) -> DatabaseResult<()> {
        let tbl = Self::table();
        let _db = db.clone().lock_owned().await;
        let db_tbl = open_table(_db, &tbl).await?;
        // let pk = Self::primary_key();
        db_tbl.delete(predicate).await.map_err(|e| {
                                           log::error!("Error: {:?}", e);
                                           DatabaseError::FailToDelete(tbl.clone())
                                       })?;
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

        let record_batch = to_record_batch(&UpdatePartial::arrow_schema().fields, &items.clone()).map_err(|e| {
                               log::error!("Error: {:?}", e);
                               DatabaseError::SerializationError
                           })?;

        let schema = Self::arrow_schema();
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
        Ok(())
    }
}
