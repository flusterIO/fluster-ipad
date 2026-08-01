use std::fmt::Display;

use arrow_array::RecordBatchIterator;
use conundrum::ecosystem::{
    db::tables::DatabaseTable,
    error_handling::db_error::{DatabaseError, DatabaseResult},
};

use crate::vector::database::{db::ArcMutexDB, db_traits::db_entity::DBEntity, open_table::open_table};

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
    // async fn get_by_predicate(predicate: Option<String>, db: &ArcMutexDB) ->
    // DatabaseResult<Vec<Self>> {

    // }
    async fn delete_by_primary_key(id: IDType, table: DatabaseTable, db: &ArcMutexDB) -> DatabaseResult<()>;
}
