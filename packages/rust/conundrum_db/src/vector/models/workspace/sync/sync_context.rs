use std::{collections::HashMap, sync::Arc};

use conundrum::{
    ecosystem::{
        db::{db_client::db_client::DBClient, db_traits::entity_crud::EntityCRUD},
        error_handling::db_error::DatabaseResult,
    },
    lang::constants::file_types::ParsableFileType,
};
use std::path::Path;
use strum::IntoEnumIterator;

use crate::vector::models::{
    taggables::{auto_taggable::AutoTaggable, subject::Subject, tag::Tag, topic::Topic},
    workspace::sync::{ir::generic_tag_input::GenericTagInput, sync_count::SyncCount},
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct SyncContext {
    pub count: SyncCount,
    pub tags: Vec<GenericTagInput>,
    pub subjects: Vec<GenericTagInput>,
    pub topics: Vec<GenericTagInput>,
    pub auto_taggables: Vec<AutoTaggable>,
}

impl SyncContext {
    pub async fn new(database: DBClient) -> DatabaseResult<Self> {
        let auto_taggables = AutoTaggable::get_by_predicate(None, None, None, database.clone()).await?;
        Ok(Self { count: SyncCount::default(),
                  tags: Vec::new(),
                  topics: Vec::new(),
                  subjects: Vec::new(),
                  auto_taggables })
    }

    pub fn append_tag(&mut self, value: GenericTagInput) {
        let exists = self.tags.iter().any(|x| x.value == value.value.clone());
        if !exists {
            self.tags.push(value);
        }
    }

    pub fn append_topic(&mut self, value: GenericTagInput) {
        let exists = self.topics.iter().any(|x| x.value == value.value.clone());
        if !exists {
            self.tags.push(value);
        }
    }

    pub fn append_subject(&mut self, value: GenericTagInput) {
        let exists = self.subjects.iter().any(|x| x.value == value.value.clone());
        if !exists {
            self.tags.push(value);
        }
    }

    pub fn matching_autotaggables<P>(&self, path: P) -> DatabaseResult<Vec<AutoTaggable>>
        where P: AsRef<Path> {
        let mut auto_taggables = Vec::new();
        for item in &self.auto_taggables {
            if item.matches_path(&path)? {
                auto_taggables.push(item.clone());
            }
        }
        Ok(auto_taggables)
    }

    async fn sync_topics(&self, db: DBClient, clean: bool) -> DatabaseResult<()> {
        let existing_tags = Topic::get_by_predicate(None, None, None, db.clone()).await?;
        todo!()
    }

    async fn sync_subjects(&self, db: DBClient, clean: bool) -> DatabaseResult<()> {
        let existing_tags = Subject::get_by_predicate(None, None, None, db.clone()).await?;
        todo!()
    }

    async fn sync_tags(&self, db: DBClient, clean: bool) -> DatabaseResult<()> {
        let existing_tags = Tag::get_by_predicate(None, None, None, db.clone()).await?;
        todo!()
    }

    pub async fn sync_taggables(&self, db: DBClient, clean_taggables: bool) -> DatabaseResult<()> {
        self.sync_tags(db.clone(), clean_taggables).await?;
        self.sync_topics(db.clone(), clean_taggables).await?;
        self.sync_subjects(db.clone(), clean_taggables).await?;
        Ok(())
    }
}
