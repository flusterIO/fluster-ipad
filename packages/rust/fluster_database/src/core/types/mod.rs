pub mod pagination;
pub mod traits;
use lancedb::Connection;
use tokio::sync::MutexGuard;

pub type FlusterDbRaw = Connection;

pub type FlusterDb<'a> = MutexGuard<'a, FlusterDbRaw>;
