use std::sync::MutexGuard;

use lancedb::Connection;

pub type FlusterDbRaw = Connection;

pub type FlusterDb<'a> = MutexGuard<'a, FlusterDbRaw>;
