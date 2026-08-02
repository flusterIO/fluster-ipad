use std::{clone::CloneToUninit, fmt::Display};

use arrow_array::RecordBatchIterator;
use conundrum::ecosystem::{
    db::tables::DatabaseTable,
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use lancedb::{
    arrow::IntoArrowStream,
    data::scannable::Scannable,
    query::{ExecutableQuery, QueryBase},
};

use crate::vector::database::{
    db::ArcMutexDB, db_traits::db_entity::DBEntity, open_table::open_table, pagination::PaginationParams,
};

pub trait EntityCRUD<IDType: Display>: DBEntity {
    async fn save_many(items: Vec<Self>, db: &ArcMutexDB) -> DatabaseResult<()>
        where Self: Sized {
        let schema = Self::arrow_schema();
        let _db = db.clone().lock_owned().await;
        let tbl = open_table(_db, Self::table()).await?;
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
    async fn get_by_predicate(predicate: Option<String>,
                              pagination: Option<PaginationParams>,
                              db: &ArcMutexDB)
                              -> DatabaseResult<Vec<Self>>
        where Self: Sized {
        let _db = db.clone().lock_owned().await;
        let tbl = open_table(_db, Self::table()).await?;
        let mut query_builder = tbl.query();
        if let Some(_predicate) = predicate {
            query_builder = query_builder.only_if(_predicate);
        }
        if let Some(_pagination) = pagination {
            query_builder = query_builder.limit(_pagination.per_page as usize)
                                         .offset(_pagination.per_page as usize * (_pagination.page as usize - 1));
        }
        let mut res = query_builder.execute().await.map_err(|e| {
                                                        log::error!("Error: {:?}", e);
                                                        DatabaseError::FailToQueryEntity(predicate)
                                                    })?;

        let mut items: Vec<Self> = Vec::new();
        todo!()

        // for batch in res.iter() {
        //     let data: Vec<FlashcardModel> =
        // from_record_batch(batch).map_err(|e| {
        // println!("Error: {:?}", e);
        // FlusterError::FailToSerialize
        // })?;     items.extend(data);
        // }
    }
    async fn delete_by_primary_key(id: IDType, table: DatabaseTable, db: &ArcMutexDB) -> DatabaseResult<()>;
}
