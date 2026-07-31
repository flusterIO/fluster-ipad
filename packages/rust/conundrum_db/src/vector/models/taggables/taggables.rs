use crate::vector::{
    database::{db::ArcMutexDB, db_traits::pure_model_instance::PureModelInstanceMethods},
    models::taggables::{subject::Subject, tag::Tag, topic::Topic},
};
use conundrum::ecosystem::error_handling::db_error::DatabaseResult;
use serde::{Deserialize, Serialize};
use surrealdb_types::RecordId;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Taggables {
    pub tags: Vec<Tag>,
    pub topic: Option<Topic>,
    pub subject: Option<Subject>,
}

impl Taggables {
    /// Returns a tuple of the tags, topic and subject record id's in that
    /// order.
    pub async fn upsert_all(&self,
                            db: &ArcMutexDB)
                            -> DatabaseResult<(Vec<RecordId>, Option<RecordId>, Option<RecordId>)> {
        let mut tag_records: Vec<RecordId> = Vec::new();
        for t in &self.tags {
            let r: RecordId = t.upsert_self(db).await?;
            tag_records.push(r);
        }
        let topic_record = match &self.topic {
            Some(s) => Some(s.upsert_self(db).await?),
            Option::None => None,
        };
        let subject_record = match &self.subject {
            Some(s) => Some(s.upsert_self(db).await?),
            Option::None => None,
        };
        Ok((tag_records, topic_record, subject_record))
    }
}
