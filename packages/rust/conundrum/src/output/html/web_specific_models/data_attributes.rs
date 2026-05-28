use serde::{Deserialize, Serialize};

/// I was being lazy before, but from now on all data-attributes can be found
/// with this enum. I'll do my best to go back and apply the one's already
/// attributed here.
#[derive(Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString)]
pub enum DataAttributeKeys {
    #[serde(rename = "data-task-status")]
    #[strum(to_string = "data-task-status")]
    TaskListCompleteStatus,
}
