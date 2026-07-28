use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

use crate::vector::models::{
    application_support::application_permission::ApplicationPermission, date_time::date_time::DateTime,
};

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct ApplicationData {
    /// A user-facing string used as a title for the application
    pub label: String,
    /// An optional string that really only makes sense if your application is a
    /// small utility that people might loose track of.
    pub desc: Option<String>,
    pub last_sync: DateTime,
    pub permissions: Vec<ApplicationPermission>,
}
