use conundrum::ecosystem::{db::db::ArcMutexDB, error_handling::db_error::DatabaseResult};

pub async fn sync_conundrum_path(fp: String, db: &ArcMutexDB) -> DatabaseResult<()> {}
