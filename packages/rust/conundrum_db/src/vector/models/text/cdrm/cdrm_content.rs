use conundrum_fs::models::user_workspace::workspace_relative_path::PureWorkspaceRelativePath;
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    ai::ai_generated_status::AIGeneratedStatus,
    date_time::date_time::DateTime,
    primitives::db_id::DatabaseId,
    taggables::{optional_taggable::OptionalTaggable, subject::Subject, tag_list::TagList, topic::Topic},
    text::cdrm::pure_cdrm_content::PureCdrmContent,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CdrmContent {
    pub id: DatabaseId,
    pub title: Option<String>,
    pub content: String,
    pub ai_generated: AIGeneratedStatus,
    pub tags: TagList,
    pub topic: OptionalTaggable<Topic>,
    pub subject: OptionalTaggable<Subject>,
    pub ws_path: Option<PureWorkspaceRelativePath>,
    pub ctime: DateTime,
    pub utime: DateTime,
}

impl CdrmContent {
    pub fn get_pure_model(&self) -> PureCdrmContent {
        PureCdrmContent { id: self.id.clone(),
                          title: self.title.clone(),
                          content: self.content.clone(),
                          ai_generated: self.ai_generated.clone(),
                          ws_path: self.ws_path.clone(),
                          ctime: self.ctime.clone(),
                          utime: self.utime.clone() }
    }
}
