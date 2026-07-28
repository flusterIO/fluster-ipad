use serde::{Deserialize, Serialize};

use crate::vector::models::{
    ai::ai_generated_status::AIGeneratedStatus,
    date_time::date_time::DateTime,
    primitives::db_id::DatabaseId,
    taggables::{optional_taggable::OptionalTaggable, subject::Subject, tag_list::TagList, topic::Topic},
    text::typst::pure_typst_model::PureTypstContent,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TypstContent {
    pub id: DatabaseId,
    pub title: Option<String>,
    pub content: String,
    pub ai_generated: AIGeneratedStatus,
    pub tags: TagList,
    pub topic: OptionalTaggable<Topic>,
    pub subject: OptionalTaggable<Subject>,
    pub fs_path: Option<String>,
    pub ctime: DateTime,
    pub utime: DateTime,
}

impl TypstContent {
    pub fn get_pure_model(&self) -> PureTypstContent {
        PureTypstContent { id: self.id.clone(),
                           title: self.title.clone(),
                           content: self.content.clone(),
                           ai_generated: self.ai_generated.clone(),
                           fs_path: self.fs_path.clone(),
                           ctime: self.ctime.clone(),
                           utime: self.utime.clone() }
    }
}
