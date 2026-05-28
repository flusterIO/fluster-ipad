use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString)]
pub enum DataAttributeKeys {
    #[serde(rename = "data-task-complete")]
    #[strum(to_string = "data-task-complete")]
    TaskListCompleteStatus,
}
