use crate::vector::models::date_time::{date_time::DateTime, schedule::time_block::TimeBlock};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub enum DateTimeOrTimeBlock {
    Date(DateTime),
    TimeBlock(TimeBlock),
}
