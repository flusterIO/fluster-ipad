use conundrum::lifted_models::primitives::date_time::DateTime;

use crate::vector::models::date_time::schedule::time_block::TimeBlock;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub enum DateTimeOrTimeBlock {
    Date(DateTime),
    TimeBlock(TimeBlock),
}
