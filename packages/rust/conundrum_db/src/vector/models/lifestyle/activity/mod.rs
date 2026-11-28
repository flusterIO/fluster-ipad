use conundrum_lang::lifted_models::primitives::{case_insensitive_string::CaseInsensitiveString, db_id::DatabaseId};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    ai::ai_interactions::AIInteractions,
    date_time::{date_time_or_timeblock::DateTimeOrTimeBlock, timeblock_or_duration::TimeBlockOrDuration},
    lifestyle::life_connections::models::{long_term_goal::LongTermGoal, short_term_goal::ShortTermGoal},
};

#[derive(Clone, Serialize, Deserialize, Debug, Dummy, specta::Type)]
pub struct Activity {
    pub id: DatabaseId,
    pub label: CaseInsensitiveString,
    pub description: Option<String>,
    /// A float between 0 and 1 indicating the productivity of this activity
    /// session.
    pub productivity: f64,
    pub associated_short_term_goals: Vec<ShortTermGoal>,
    pub associated_long_term_goals: Vec<LongTermGoal>,
    pub ai: AIInteractions,
    pub time_or_duration: TimeBlockOrDuration,
}
