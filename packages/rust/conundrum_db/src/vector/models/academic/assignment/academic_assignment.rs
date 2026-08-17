use conundrum::lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    academic::assignment::milestone::milestone::Milestone,
    date_time::alarm::alarm::Alarm,
    taggables::{subject::Subject, tag::Tag, topic::Topic},
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct Assignment {
    pub id: DatabaseId,
    pub label: String,
    pub desc: Option<String>,
    pub tags: Vec<Tag>,
    pub topic: Option<Topic>,
    pub subject: Option<Subject>,
    pub milestones: Vec<Milestone>,
    pub due_at: Option<DateTime>,
    pub alarms: Vec<Alarm>,
    pub ctime: DateTime,
    pub utime: DateTime,
}
