use conundrum::lifted_models::primitives::date_time::DateTime;
use serde::{Deserialize, Serialize};

use crate::vector::models::application_support::{
    application_permission::ApplicationPermission, greeting_generation_strategy::GreetingGenerationStrategy,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApplicationData {
    /// A user-facing string used as a title for the application
    pub label: String,
    /// An optional string that really only makes sense if your application is a
    /// small utility that people might loose track of.
    pub desc: Option<String>,
    pub last_sync: DateTime,
    pub permissions: Vec<ApplicationPermission>,
    pub dynamic_greeting: GreetingGenerationStrategy,
}
